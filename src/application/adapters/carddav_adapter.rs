use quick_xml::{
    Reader, Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
/**
 * CardDAV Adapter Module
 *
 * This module provides conversion between CardDAV protocol XML structures and
 * OxiCloud domain objects. It handles parsing CardDAV request XML and generating
 * CardDAV response XML according to RFC 6352.
 */
use std::io::{BufReader, Read, Write};

use crate::application::adapters::webdav_adapter::{
    PropFindRequest, PropFindType, QualifiedName, Result, WebDavAdapter, WebDavError,
};
use crate::application::dtos::address_book_dto::AddressBookDto;
use crate::application::dtos::contact_dto::ContactDto;

/// CardDAV report type
#[derive(Debug, PartialEq)]
pub enum CardDavReportType {
    /// Addressbook-query report
    AddressbookQuery { props: Vec<QualifiedName> },
    /// Addressbook-multiget report
    AddressbookMultiget {
        hrefs: Vec<String>,
        props: Vec<QualifiedName>,
    },
    /// Sync-collection report
    SyncCollection {
        sync_token: String,
        props: Vec<QualifiedName>,
    },
}

/// CardDAV adapter for XML parsing/generation
pub struct CardDavAdapter;

impl CardDavAdapter {
    /// Parse a REPORT XML request for CardDAV
    pub fn parse_report<R: Read>(reader: R) -> Result<CardDavReportType> {
        let mut xml_reader = Reader::from_reader(BufReader::new(reader));
        xml_reader.config_mut().trim_text(true);

        let mut buffer = Vec::new();
        let mut in_addressbook_query = false;
        let mut in_addressbook_multiget = false;
        let mut in_sync_collection = false;
        let mut in_prop = false;
        let mut props = Vec::new();
        let mut hrefs = Vec::new();
        let mut sync_token = String::new();
        let mut in_href = false;
        let mut in_sync_token = false;

        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "addressbook-query" || s.ends_with(":addressbook-query") => {
                            in_addressbook_query = true
                        }
                        s if s == "addressbook-multiget"
                            || s.ends_with(":addressbook-multiget") =>
                        {
                            in_addressbook_multiget = true
                        }
                        s if s == "sync-collection" || s.ends_with(":sync-collection") => {
                            in_sync_collection = true
                        }
                        s if s == "prop" || s.ends_with(":prop") => in_prop = true,
                        s if s == "href" || s.ends_with(":href") => in_href = true,
                        s if s == "sync-token" || s.ends_with(":sync-token") => {
                            in_sync_token = true
                        }
                        _ if in_prop => {
                            let namespace = WebDavAdapter::extract_namespace(name_str);
                            let prop_name = WebDavAdapter::extract_local_name(name_str);
                            props.push(QualifiedName::new(namespace, prop_name));
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.decode().unwrap_or_default();
                    if in_href {
                        hrefs.push(text.to_string());
                    } else if in_sync_token {
                        sync_token = text.to_string();
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "prop" || s.ends_with(":prop") => in_prop = false,
                        s if s == "href" || s.ends_with(":href") => in_href = false,
                        s if s == "sync-token" || s.ends_with(":sync-token") => {
                            in_sync_token = false
                        }
                        _ => {}
                    }
                }
                Ok(Event::Empty(ref e)) if in_prop => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");
                    let namespace = WebDavAdapter::extract_namespace(name_str);
                    let prop_name = WebDavAdapter::extract_local_name(name_str);
                    props.push(QualifiedName::new(namespace, prop_name));
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(e)),
                _ => (),
            }
            buffer.clear();
        }

        if in_addressbook_multiget || !hrefs.is_empty() {
            Ok(CardDavReportType::AddressbookMultiget { hrefs, props })
        } else if in_sync_collection {
            Ok(CardDavReportType::SyncCollection { sync_token, props })
        } else if in_addressbook_query {
            Ok(CardDavReportType::AddressbookQuery { props })
        } else {
            // Default
            Ok(CardDavReportType::AddressbookQuery { props })
        }
    }

    /// Generate a PROPFIND response listing address books
    pub fn generate_addressbooks_propfind_response<W: Write>(
        writer: W,
        address_books: &[AddressBookDto],
        request: &PropFindRequest,
        base_href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([
                ("xmlns:D", "DAV:"),
                ("xmlns:CR", "urn:ietf:params:xml:ns:carddav"),
                ("xmlns:CS", "http://calendarserver.org/ns/"),
            ]),
        ))?;

        for book in address_books {
            Self::write_addressbook_response(
                &mut xml_writer,
                book,
                request,
                &format!("{}{}/", base_href, book.id),
            )?;
        }

        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;
        Ok(())
    }

    /// Generate PROPFIND for a single address book collection + contacts
    pub fn generate_addressbook_collection_propfind<W: Write>(
        writer: W,
        address_book: &AddressBookDto,
        contacts: &[ContactDto],
        request: &PropFindRequest,
        base_href: &str,
        depth: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([
                ("xmlns:D", "DAV:"),
                ("xmlns:CR", "urn:ietf:params:xml:ns:carddav"),
                ("xmlns:CS", "http://calendarserver.org/ns/"),
            ]),
        ))?;

        // Write the address book itself
        Self::write_addressbook_response(&mut xml_writer, address_book, request, base_href)?;

        // Write contacts if depth > 0
        if depth != "0" {
            for contact in contacts {
                let contact_href = format!("{}{}.vcf", base_href, contact.uid);
                Self::write_contact_response(&mut xml_writer, contact, &[], &contact_href)?;
            }
        }

        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;
        Ok(())
    }

    /// Write address book properties
    fn write_addressbook_response<W: Write>(
        xml_writer: &mut Writer<W>,
        book: &AddressBookDto,
        request: &PropFindRequest,
        href: &str,
    ) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

        xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;
        xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

        match &request.prop_find_type {
            PropFindType::AllProp => Self::write_addressbook_all_props(xml_writer, book)?,
            PropFindType::PropName => Self::write_addressbook_prop_names(xml_writer)?,
            PropFindType::Prop(props) => {
                Self::write_addressbook_requested_props(xml_writer, book, props)?
            }
        }

        xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

        xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
        xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

        xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;

        Ok(())
    }

    fn write_addressbook_all_props<W: Write>(
        xml_writer: &mut Writer<W>,
        book: &AddressBookDto,
    ) -> Result<()> {
        // resourcetype: collection + addressbook
        xml_writer.write_event(Event::Start(BytesStart::new("D:resourcetype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:collection")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("CR:addressbook")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:resourcetype")))?;

        // displayname
        xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&book.name)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;

        // getlastmodified
        xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&book.updated_at.to_rfc2822())))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;

        // getetag
        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Text(BytesText::new(&format!("\"{}\"", book.id))))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;

        // getcontenttype
        xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
        xml_writer.write_event(Event::Text(BytesText::new("text/vcard")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;

        // supported-address-data
        xml_writer.write_event(Event::Start(BytesStart::new("CR:supported-address-data")))?;
        xml_writer.write_event(Event::Empty(
            BytesStart::new("CR:address-data-type")
                .with_attributes([("content-type", "text/vcard"), ("version", "3.0")]),
        ))?;
        xml_writer.write_event(Event::Empty(
            BytesStart::new("CR:address-data-type")
                .with_attributes([("content-type", "text/vcard"), ("version", "4.0")]),
        ))?;
        xml_writer.write_event(Event::End(BytesEnd::new("CR:supported-address-data")))?;

        // addressbook-description
        if let Some(ref desc) = book.description {
            xml_writer.write_event(Event::Start(BytesStart::new("CR:addressbook-description")))?;
            xml_writer.write_event(Event::Text(BytesText::new(desc)))?;
            xml_writer.write_event(Event::End(BytesEnd::new("CR:addressbook-description")))?;
        }

        // current-user-privilege-set
        xml_writer.write_event(Event::Start(BytesStart::new(
            "D:current-user-privilege-set",
        )))?;
        xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:read")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;
        xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:write")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:current-user-privilege-set")))?;

        Ok(())
    }

    fn write_addressbook_prop_names<W: Write>(xml_writer: &mut Writer<W>) -> Result<()> {
        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:displayname")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getlastmodified")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getetag")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("D:getcontenttype")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("CR:supported-address-data")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new("CR:addressbook-description")))?;
        xml_writer.write_event(Event::Empty(BytesStart::new(
            "D:current-user-privilege-set",
        )))?;
        Ok(())
    }

    fn write_addressbook_requested_props<W: Write>(
        xml_writer: &mut Writer<W>,
        book: &AddressBookDto,
        props: &[QualifiedName],
    ) -> Result<()> {
        for prop in props {
            match (prop.namespace.as_str(), prop.name.as_str()) {
                ("DAV:", "resourcetype") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:resourcetype")))?;
                    xml_writer.write_event(Event::Empty(BytesStart::new("D:collection")))?;
                    xml_writer.write_event(Event::Empty(BytesStart::new("CR:addressbook")))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:resourcetype")))?;
                }
                ("DAV:", "displayname") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:displayname")))?;
                    xml_writer.write_event(Event::Text(BytesText::new(&book.name)))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:displayname")))?;
                }
                ("DAV:", "getlastmodified") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
                    xml_writer
                        .write_event(Event::Text(BytesText::new(&book.updated_at.to_rfc2822())))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;
                }
                ("DAV:", "getetag") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
                    xml_writer
                        .write_event(Event::Text(BytesText::new(&format!("\"{}\"", book.id))))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;
                }
                ("DAV:", "getcontenttype") => {
                    xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
                    xml_writer.write_event(Event::Text(BytesText::new("text/vcard")))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;
                }
                ("urn:ietf:params:xml:ns:carddav", "addressbook-description") => {
                    if let Some(ref desc) = book.description {
                        xml_writer.write_event(Event::Start(BytesStart::new(
                            "CR:addressbook-description",
                        )))?;
                        xml_writer.write_event(Event::Text(BytesText::new(desc)))?;
                        xml_writer
                            .write_event(Event::End(BytesEnd::new("CR:addressbook-description")))?;
                    } else {
                        xml_writer.write_event(Event::Empty(BytesStart::new(
                            "CR:addressbook-description",
                        )))?;
                    }
                }
                ("urn:ietf:params:xml:ns:carddav", "supported-address-data") => {
                    xml_writer
                        .write_event(Event::Start(BytesStart::new("CR:supported-address-data")))?;
                    xml_writer.write_event(Event::Empty(
                        BytesStart::new("CR:address-data-type")
                            .with_attributes([("content-type", "text/vcard"), ("version", "3.0")]),
                    ))?;
                    xml_writer
                        .write_event(Event::End(BytesEnd::new("CR:supported-address-data")))?;
                }
                ("DAV:", "current-user-privilege-set") => {
                    xml_writer.write_event(Event::Start(BytesStart::new(
                        "D:current-user-privilege-set",
                    )))?;
                    xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
                    xml_writer.write_event(Event::Empty(BytesStart::new("D:read")))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;
                    xml_writer.write_event(Event::Start(BytesStart::new("D:privilege")))?;
                    xml_writer.write_event(Event::Empty(BytesStart::new("D:write")))?;
                    xml_writer.write_event(Event::End(BytesEnd::new("D:privilege")))?;
                    xml_writer
                        .write_event(Event::End(BytesEnd::new("D:current-user-privilege-set")))?;
                }
                _ => {
                    let prop_name = if prop.namespace == "urn:ietf:params:xml:ns:carddav" {
                        format!("CR:{}", prop.name)
                    } else if prop.namespace == "DAV:" {
                        format!("D:{}", prop.name)
                    } else {
                        prop.name.clone()
                    };
                    xml_writer.write_event(Event::Empty(BytesStart::new(&prop_name)))?;
                }
            }
        }
        Ok(())
    }

    /// Generate response for contacts (for REPORT)
    pub fn generate_contacts_response<W: Write>(
        writer: W,
        contacts: &[ContactDto],
        vcards: &[(String, String)], // (uid, vcard_data)
        report: &CardDavReportType,
        base_href: &str,
    ) -> Result<()> {
        let mut xml_writer = Writer::new(writer);

        xml_writer.write_event(Event::Start(
            BytesStart::new("D:multistatus").with_attributes([
                ("xmlns:D", "DAV:"),
                ("xmlns:CR", "urn:ietf:params:xml:ns:carddav"),
            ]),
        ))?;

        let props = match report {
            CardDavReportType::AddressbookQuery { props } => props.clone(),
            CardDavReportType::AddressbookMultiget { props, .. } => props.clone(),
            CardDavReportType::SyncCollection { props, .. } => props.clone(),
        };

        for contact in contacts {
            let href = format!("{}{}.vcf", base_href, contact.uid);
            let vcard = vcards
                .iter()
                .find(|(uid, _)| *uid == contact.uid)
                .map(|(_, data)| data.as_str())
                .unwrap_or("");
            Self::write_contact_response(&mut xml_writer, contact, &props, &href)?;
            // If address-data is requested, include vcard
            if props.iter().any(|p| p.name == "address-data") || props.is_empty() {
                // Already handled in write_contact_response
            }
            let _ = vcard; // suppress warning - used via contact_to_vcard fallback
        }

        xml_writer.write_event(Event::End(BytesEnd::new("D:multistatus")))?;
        Ok(())
    }

    /// Write a single contact response element
    fn write_contact_response<W: Write>(
        xml_writer: &mut Writer<W>,
        contact: &ContactDto,
        props: &[QualifiedName],
        href: &str,
    ) -> Result<()> {
        xml_writer.write_event(Event::Start(BytesStart::new("D:response")))?;

        xml_writer.write_event(Event::Start(BytesStart::new("D:href")))?;
        xml_writer.write_event(Event::Text(BytesText::new(href)))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:href")))?;

        xml_writer.write_event(Event::Start(BytesStart::new("D:propstat")))?;
        xml_writer.write_event(Event::Start(BytesStart::new("D:prop")))?;

        if props.is_empty() {
            // Return standard properties
            xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;

            xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
            xml_writer.write_event(Event::Text(BytesText::new(&format!(
                "\"{}\"",
                contact.etag
            ))))?;
            xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;

            xml_writer.write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
            xml_writer.write_event(Event::Text(BytesText::new("text/vcard; charset=utf-8")))?;
            xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;

            // Include vCard data
            let vcard = contact_to_vcard(contact);
            xml_writer.write_event(Event::Start(BytesStart::new("CR:address-data")))?;
            xml_writer.write_event(Event::Text(BytesText::new(&vcard)))?;
            xml_writer.write_event(Event::End(BytesEnd::new("CR:address-data")))?;
        } else {
            for prop in props {
                match (prop.namespace.as_str(), prop.name.as_str()) {
                    ("DAV:", "resourcetype") => {
                        xml_writer.write_event(Event::Empty(BytesStart::new("D:resourcetype")))?;
                    }
                    ("DAV:", "getetag") => {
                        xml_writer.write_event(Event::Start(BytesStart::new("D:getetag")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(&format!(
                            "\"{}\"",
                            contact.etag
                        ))))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getetag")))?;
                    }
                    ("DAV:", "getcontenttype") => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getcontenttype")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(
                            "text/vcard; charset=utf-8",
                        )))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getcontenttype")))?;
                    }
                    ("DAV:", "getlastmodified") => {
                        xml_writer
                            .write_event(Event::Start(BytesStart::new("D:getlastmodified")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(
                            &contact.updated_at.to_rfc2822(),
                        )))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("D:getlastmodified")))?;
                    }
                    ("urn:ietf:params:xml:ns:carddav", "address-data") => {
                        let vcard = contact_to_vcard(contact);
                        xml_writer.write_event(Event::Start(BytesStart::new("CR:address-data")))?;
                        xml_writer.write_event(Event::Text(BytesText::new(&vcard)))?;
                        xml_writer.write_event(Event::End(BytesEnd::new("CR:address-data")))?;
                    }
                    _ => {
                        let prop_name = if prop.namespace == "urn:ietf:params:xml:ns:carddav" {
                            format!("CR:{}", prop.name)
                        } else if prop.namespace == "DAV:" {
                            format!("D:{}", prop.name)
                        } else {
                            prop.name.clone()
                        };
                        xml_writer.write_event(Event::Empty(BytesStart::new(&prop_name)))?;
                    }
                }
            }
        }

        xml_writer.write_event(Event::End(BytesEnd::new("D:prop")))?;

        xml_writer.write_event(Event::Start(BytesStart::new("D:status")))?;
        xml_writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:status")))?;

        xml_writer.write_event(Event::End(BytesEnd::new("D:propstat")))?;
        xml_writer.write_event(Event::End(BytesEnd::new("D:response")))?;

        Ok(())
    }

    /// Parse a MKCOL XML request for making an address book
    pub fn parse_mkaddressbook<R: Read>(
        reader: R,
    ) -> Result<(String, Option<String>, Option<String>)> {
        let mut xml_reader = Reader::from_reader(BufReader::new(reader));
        xml_reader.config_mut().trim_text(true);

        let mut buffer = Vec::new();
        let mut in_set = false;
        let mut in_prop = false;
        let mut in_displayname = false;
        let mut in_description = false;
        let mut in_color = false;

        let mut displayname = String::new();
        let mut description = None;
        let mut color = None;

        loop {
            match xml_reader.read_event_into(&mut buffer) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");

                    match name_str {
                        s if s == "set" || s.ends_with(":set") => in_set = true,
                        s if in_set && (s == "prop" || s.ends_with(":prop")) => in_prop = true,
                        s if in_prop && (s == "displayname" || s.ends_with(":displayname")) => {
                            in_displayname = true
                        }
                        s if in_prop
                            && (s == "addressbook-description"
                                || s.ends_with(":addressbook-description")) =>
                        {
                            in_description = true
                        }
                        s if in_prop && (s.contains("color")) => in_color = true,
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = e.decode().unwrap_or_default();
                    if in_displayname {
                        displayname = text.to_string();
                    } else if in_description {
                        description = Some(text.to_string());
                    } else if in_color {
                        color = Some(text.to_string());
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = std::str::from_utf8(name.as_ref()).unwrap_or("");
                    match name_str {
                        s if s == "set" || s.ends_with(":set") => in_set = false,
                        s if s == "prop" || s.ends_with(":prop") => in_prop = false,
                        s if s == "displayname" || s.ends_with(":displayname") => {
                            in_displayname = false
                        }
                        s if s.contains("description") => in_description = false,
                        s if s.contains("color") => in_color = false,
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(WebDavError::XmlError(e)),
                _ => (),
            }
            buffer.clear();
        }

        if displayname.is_empty() {
            displayname = format!("Address Book {}", uuid::Uuid::new_v4());
        }

        Ok((displayname, description, color))
    }
}

/// Convert a ContactDto to vCard 3.0 format
pub fn contact_to_vcard(contact: &ContactDto) -> String {
    let mut vcard = String::from("BEGIN:VCARD\r\nVERSION:3.0\r\n");

    vcard.push_str(&format!("UID:{}\r\n", contact.uid));

    if let (Some(last), Some(first)) = (&contact.last_name, &contact.first_name) {
        vcard.push_str(&format!("N:{};{};;;\r\n", last, first));
    } else if let Some(last) = &contact.last_name {
        vcard.push_str(&format!("N:{};;;;\r\n", last));
    } else if let Some(first) = &contact.first_name {
        vcard.push_str(&format!("N:;{};;;\r\n", first));
    }

    if let Some(fn_name) = &contact.full_name {
        vcard.push_str(&format!("FN:{}\r\n", fn_name));
    } else {
        // FN is mandatory in vCard 3.0
        let fn_name = format!(
            "{} {}",
            contact.first_name.as_deref().unwrap_or(""),
            contact.last_name.as_deref().unwrap_or(""),
        )
        .trim()
        .to_string();
        if !fn_name.is_empty() {
            vcard.push_str(&format!("FN:{}\r\n", fn_name));
        } else {
            vcard.push_str("FN:Unknown\r\n");
        }
    }

    if let Some(nickname) = &contact.nickname {
        vcard.push_str(&format!("NICKNAME:{}\r\n", nickname));
    }

    for email in &contact.email {
        vcard.push_str(&format!(
            "EMAIL;TYPE={}:{}\r\n",
            email.r#type.to_uppercase(),
            email.email
        ));
    }

    for phone in &contact.phone {
        vcard.push_str(&format!(
            "TEL;TYPE={}:{}\r\n",
            phone.r#type.to_uppercase(),
            phone.number
        ));
    }

    for addr in &contact.address {
        let adr = format!(
            ";;{};{};{};{};{}",
            addr.street.as_deref().unwrap_or(""),
            addr.city.as_deref().unwrap_or(""),
            addr.state.as_deref().unwrap_or(""),
            addr.postal_code.as_deref().unwrap_or(""),
            addr.country.as_deref().unwrap_or(""),
        );
        vcard.push_str(&format!(
            "ADR;TYPE={}:{}\r\n",
            addr.r#type.to_uppercase(),
            adr
        ));
    }

    if let Some(org) = &contact.organization {
        vcard.push_str(&format!("ORG:{}\r\n", org));
    }
    if let Some(title) = &contact.title {
        vcard.push_str(&format!("TITLE:{}\r\n", title));
    }
    if let Some(notes) = &contact.notes {
        vcard.push_str(&format!("NOTE:{}\r\n", notes.replace('\n', "\\n")));
    }
    if let Some(bday) = &contact.birthday {
        vcard.push_str(&format!("BDAY:{}\r\n", bday.format("%Y-%m-%d")));
    }
    if let Some(photo) = &contact.photo_url {
        vcard.push_str(&format!("PHOTO;VALUE=URI:{}\r\n", photo));
    }

    vcard.push_str(&format!(
        "REV:{}\r\n",
        contact.updated_at.format("%Y%m%dT%H%M%SZ")
    ));
    vcard.push_str("END:VCARD\r\n");

    vcard
}
