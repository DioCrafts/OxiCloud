let e={items:[],filteredItems:[],currentItem:null,_headers(e=!1){let t={},i=localStorage.getItem("oxicloud_token");return i&&(t.Authorization=`Bearer ${i}`),e&&(t["Content-Type"]="application/json"),t;},init(){console.log("Initializing shared view component (API-backed)"),this.loadItems();},show(){this.displayUI(),this.attachEventListeners(),this.loadItems().then(()=>this.filterAndSortItems());},hide(){let e=document.getElementById("shared-container");e&&(e.style.display="none");},async loadItems(){try{let e=await fetch("/api/shares?page=1&per_page=1000",{headers:this._headers()});if(e.ok){let t=await e.json();this.items=t.items||[];}else this.items=[];}catch(e){console.error("Error loading shared items:",e),this.items=[];}this.filteredItems=[...this.items];},displayUI(){let e=document.querySelector(".content-area"),t=document.getElementById("shared-container");!t&&((t=document.createElement("div")).id="shared-container",t.className="shared-view-container",e&&e.appendChild(t)),t.style.display="block",t.innerHTML=`
            <div class="shared-header">
                <div class="shared-filters">
                    <select id="filter-type" class="shared-filter-select">
                        <option value="all" data-i18n="shared_filterAll">All</option>
                        <option value="file" data-i18n="shared_filterFiles">Files</option>
                        <option value="folder" data-i18n="shared_filterFolders">Folders</option>
                    </select>
                    <select id="sort-by" class="shared-filter-select">
                        <option value="date" data-i18n="shared_sortByDate">Sort by date</option>
                        <option value="name" data-i18n="shared_sortByName">Sort by name</option>
                        <option value="expiration" data-i18n="shared_sortByExpiration">Sort by expiration</option>
                    </select>
                </div>
            </div>

            <div id="empty-shared-state" class="empty-state" style="display:none;">
                <div class="empty-state-icon">📤</div>
                <h3 data-i18n="shared_emptyStateTitle">No shared items</h3>
                <p data-i18n="shared_emptyStateDesc">Items you share will appear here</p>
                <button id="empty-go-to-files" class="button primary" data-i18n="shared_goToFiles">Go to Files</button>
            </div>

            <div class="shared-list-container" style="display:none;">
                <table class="shared-table">
                    <thead>
                        <tr>
                            <th data-i18n="shared_colName">Name</th>
                            <th data-i18n="shared_colType">Type</th>
                            <th data-i18n="shared_colDateShared">Date</th>
                            <th data-i18n="shared_colExpiration">Expiration</th>
                            <th data-i18n="shared_colPermissions">Permissions</th>
                            <th data-i18n="shared_colPassword">Password</th>
                            <th data-i18n="shared_colActions">Actions</th>
                        </tr>
                    </thead>
                    <tbody id="shared-items-list"></tbody>
                </table>
            </div>

            <!-- Share Edit Dialog (sharedView-specific) -->
            <div id="shared-view-edit-dialog" class="shared-dialog">
                <div class="shared-dialog-content">
                    <div class="shared-dialog-header">
                        <span id="sv-dialog-icon">📄</span>
                        <span id="sv-dialog-name">Item</span>
                        <button class="close-dialog-btn">&times;</button>
                    </div>
                    <div class="share-link-section">
                        <label data-i18n="share.linkLabel">Share Link:</label>
                        <div class="share-link-input">
                            <input type="text" id="sv-share-link-url" readonly>
                            <button id="sv-copy-link-btn" class="button" data-i18n="share.copyLink">Copy</button>
                        </div>
                    </div>
                    <div class="share-permissions-section">
                        <h4 data-i18n="share.permissions">Permissions</h4>
                        <label><input type="checkbox" id="sv-permission-read" checked> <span data-i18n="share.permissionRead">Read</span></label>
                        <label><input type="checkbox" id="sv-permission-write"> <span data-i18n="share.permissionWrite">Write</span></label>
                        <label><input type="checkbox" id="sv-permission-reshare"> <span data-i18n="share.permissionReshare">Reshare</span></label>
                    </div>
                    <div class="share-password-section">
                        <label><input type="checkbox" id="sv-enable-password"> <span data-i18n="share.enablePassword">Password protection</span></label>
                        <div class="password-input-group">
                            <input type="text" id="sv-share-password" disabled placeholder="Enter password">
                            <button id="sv-generate-password" class="button small" data-i18n="share.generatePassword">Generate</button>
                        </div>
                    </div>
                    <div class="share-expiration-section">
                        <label><input type="checkbox" id="sv-enable-expiration"> <span data-i18n="share.enableExpiration">Set expiration</span></label>
                        <input type="date" id="sv-share-expiration" disabled>
                    </div>
                    <div class="share-actions">
                        <button id="sv-update-share-btn" class="button primary" data-i18n="share.update">Update</button>
                        <button id="sv-remove-share-btn" class="button danger" data-i18n="share.remove">Remove Share</button>
                    </div>
                </div>
            </div>

            <!-- Notification Dialog (sharedView-specific) -->
            <div id="sv-notification-dialog" class="shared-dialog">
                <div class="shared-dialog-content">
                    <div class="shared-dialog-header">
                        <span id="sv-notify-dialog-icon">📧</span>
                        <span id="sv-notify-dialog-name">Item</span>
                        <button class="close-dialog-btn">&times;</button>
                    </div>
                    <div class="notification-form">
                        <div class="form-group">
                            <label data-i18n="share.notifyEmail">Email:</label>
                            <input type="email" id="sv-notification-email" placeholder="recipient@example.com">
                        </div>
                        <div class="form-group">
                            <label data-i18n="share.notifyMessage">Message (optional):</label>
                            <textarea id="sv-notification-message" rows="3"></textarea>
                        </div>
                    </div>
                    <div class="notification-actions">
                        <button id="sv-send-notification-btn" class="button primary" data-i18n="share.notifySend">Send Notification</button>
                    </div>
                </div>
            </div>
        `;let i=document.getElementById("files-grid"),a=document.getElementById("files-list-view");i&&(i.style.display="none"),a&&(a.style.display="none"),window.i18n&&window.i18n.translateElement&&window.i18n.translateElement(t);},attachEventListeners(){let e=document.getElementById("filter-type"),t=document.getElementById("sort-by"),i=document.getElementById("empty-go-to-files");e&&e.addEventListener("change",()=>this.filterAndSortItems()),t&&t.addEventListener("change",()=>this.filterAndSortItems()),i&&i.addEventListener("click",()=>window.switchToFilesView());let a=document.getElementById("shared-view-edit-dialog");if(a){let e=a.querySelector(".close-dialog-btn");e&&e.addEventListener("click",()=>this.closeShareDialog());let t=document.getElementById("sv-copy-link-btn");t&&t.addEventListener("click",()=>this.copyShareLink());let i=document.getElementById("sv-enable-password"),s=document.getElementById("sv-share-password");i&&i.addEventListener("change",()=>{s&&(s.disabled=!i.checked,i.checked&&s.focus());});let n=document.getElementById("sv-generate-password");n&&n.addEventListener("click",()=>this.generatePassword());let o=document.getElementById("sv-enable-expiration"),d=document.getElementById("sv-share-expiration");o&&o.addEventListener("change",()=>{d&&(d.disabled=!o.checked,o.checked&&d.focus());});let r=document.getElementById("sv-update-share-btn");r&&r.addEventListener("click",()=>this.updateSharedItem());let l=document.getElementById("sv-remove-share-btn");l&&l.addEventListener("click",()=>this.removeSharedItem());}let s=document.getElementById("sv-notification-dialog");if(s){let e=s.querySelector(".close-dialog-btn");e&&e.addEventListener("click",()=>this.closeNotificationDialog());let t=document.getElementById("sv-send-notification-btn");t&&t.addEventListener("click",()=>this.sendNotification());}},filterAndSortItems(){let e=document.getElementById("filter-type"),t=document.getElementById("sort-by"),i=e?e.value:"all",a=t?t.value:"date",s=document.getElementById("shared-search"),n=s?s.value.toLowerCase():"";this.filteredItems=this.items.filter(e=>("all"===i||e.item_type===i)&&(e.item_name||e.item_id||"").toLowerCase().includes(n)),this.filteredItems.sort((e,t)=>"name"===a?(e.item_name||e.item_id||"").localeCompare(t.item_name||t.item_id||""):"date"===a?(t.created_at||0)-(e.created_at||0):"expiration"===a?e.expires_at||t.expires_at?e.expires_at?t.expires_at?e.expires_at-t.expires_at:-1:1:0:0),this.displaySharedItems();},displaySharedItems(){let e=document.getElementById("shared-items-list"),t=document.getElementById("empty-shared-state"),i=document.querySelector(".shared-list-container");if(e&&t&&i){if(e.innerHTML="",0===this.filteredItems.length){t.style.display="flex",i.style.display="none";return;}t.style.display="none",i.style.display="block",this.filteredItems.forEach(t=>{let i=document.createElement("tr"),a=t.item_name||t.item_id||"Unknown",s=document.createElement("td");s.className="shared-item-name";let n=document.createElement("span");n.className="item-icon",n.textContent="file"===t.item_type?"📄":"📁";let o=document.createElement("span");o.textContent=a,s.appendChild(n),s.appendChild(o);let d=document.createElement("td");d.textContent="file"===t.item_type?this.translate("shared_typeFile","File"):this.translate("shared_typeFolder","Folder");let r=document.createElement("td");r.textContent=this.formatDate(t.created_at);let l=document.createElement("td");l.textContent=t.expires_at?this.formatDate(t.expires_at):this.translate("shared_noExpiration","No expiration");let c=document.createElement("td"),h=[];t.permissions?.read&&h.push(this.translate("share_permissionRead","Read")),t.permissions?.write&&h.push(this.translate("share_permissionWrite","Write")),t.permissions?.reshare&&h.push(this.translate("share_permissionReshare","Reshare")),c.textContent=h.join(", ")||"Read";let m=document.createElement("td");m.textContent=t.has_password?this.translate("shared_hasPassword","Yes"):this.translate("shared_noPassword","No");let p=document.createElement("td");p.className="shared-item-actions";let u=document.createElement("button");u.className="action-btn edit-btn",u.innerHTML='<span class="action-icon">✏️</span>',u.title=this.translate("shared_editShare","Edit Share"),u.addEventListener("click",()=>this.openShareDialog(t));let v=document.createElement("button");v.className="action-btn notify-btn",v.innerHTML='<span class="action-icon">📧</span>',v.title=this.translate("shared_notifyShare","Notify Someone"),v.addEventListener("click",()=>this.openNotificationDialog(t));let y=document.createElement("button");y.className="action-btn copy-btn",y.innerHTML='<span class="action-icon">📋</span>',y.title=this.translate("shared_copyLink","Copy Link"),y.addEventListener("click",()=>{navigator.clipboard.writeText(t.url).then(()=>this.showNotification(this.translate("shared_linkCopied","Link copied!"))).catch(()=>this.showNotification(this.translate("shared_linkCopyFailed","Failed to copy link"),"error"));});let g=document.createElement("button");g.className="action-btn remove-btn",g.innerHTML='<span class="action-icon">🗑️</span>',g.title=this.translate("shared_removeShare","Remove Share"),g.addEventListener("click",()=>{this.currentItem=t,this.removeSharedItem();}),p.append(u,v,y,g),i.append(s,d,r,l,c,m,p),e.appendChild(i);});}},openShareDialog(e){this.currentItem=e;let t=document.getElementById("shared-view-edit-dialog"),i=e.item_name||e.item_id||"Unknown",a=document.getElementById("sv-dialog-icon"),s=document.getElementById("sv-dialog-name"),n=document.getElementById("sv-share-link-url"),o=document.getElementById("sv-enable-password"),d=document.getElementById("sv-share-password"),r=document.getElementById("sv-enable-expiration"),l=document.getElementById("sv-share-expiration"),c=document.getElementById("sv-permission-read"),h=document.getElementById("sv-permission-write"),m=document.getElementById("sv-permission-reshare");t&&(a&&(a.textContent="file"===e.item_type?"📄":"📁"),s&&(s.textContent=i),n&&(n.value=e.url||""),c&&(c.checked=e.permissions?.read!==!1),h&&(h.checked=!!e.permissions?.write),m&&(m.checked=!!e.permissions?.reshare),o&&(o.checked=e.has_password,d&&(d.disabled=!o.checked,d.value="")),r&&(r.checked=!!e.expires_at,l&&(l.disabled=!r.checked,l.value=e.expires_at?new Date(1e3*e.expires_at).toISOString().split("T")[0]:"")),t.classList.add("active"));},closeShareDialog(){let e=document.getElementById("shared-view-edit-dialog");e&&e.classList.remove("active"),this.currentItem=null;},openNotificationDialog(e){this.currentItem=e;let t=e.item_name||e.item_id||"Unknown",i=document.getElementById("sv-notification-dialog"),a=document.getElementById("sv-notify-dialog-icon"),s=document.getElementById("sv-notify-dialog-name"),n=document.getElementById("sv-notification-email"),o=document.getElementById("sv-notification-message");i&&(a&&(a.textContent="file"===e.item_type?"📄":"📁"),s&&(s.textContent=t),n&&(n.value=""),o&&(o.value=""),i.classList.add("active"));},closeNotificationDialog(){let e=document.getElementById("sv-notification-dialog");e&&e.classList.remove("active"),this.currentItem=null;},copyShareLink(){let e=document.getElementById("sv-share-link-url");e&&navigator.clipboard.writeText(e.value).then(()=>this.showNotification(this.translate("shared_linkCopied","Link copied!"))).catch(()=>this.showNotification(this.translate("shared_linkCopyFailed","Failed to copy link"),"error"));},generatePassword(){let e=document.getElementById("sv-share-password"),t=document.getElementById("sv-enable-password");if(!e||!t)return;let i="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*",a=new Uint32Array(16);crypto.getRandomValues(a);let s="";for(let e=0;e<16;e++)s+=i[a[e]%i.length];e.value=s,t.checked=!0,e.disabled=!1;},async updateSharedItem(){if(!this.currentItem)return;let e=document.getElementById("sv-permission-read"),t=document.getElementById("sv-permission-write"),i=document.getElementById("sv-permission-reshare"),a=document.getElementById("sv-enable-password"),s=document.getElementById("sv-share-password"),n=document.getElementById("sv-enable-expiration"),o=document.getElementById("sv-share-expiration"),d={permissions:{read:!e||e.checked,write:!!t&&t.checked,reshare:!!i&&i.checked},password:a&&a.checked&&s&&s.value?s.value:null,expires_at:n&&n.checked&&o&&o.value?Math.floor(new Date(o.value).getTime()/1e3):null};try{let e=await fetch(`/api/shares/${this.currentItem.id}`,{method:"PUT",headers:this._headers(!0),body:JSON.stringify(d)});if(!e.ok){let t=await e.json().catch(()=>({}));throw Error(t.error||`Server error ${e.status}`);}this.showNotification(this.translate("shared_itemUpdated","Share settings updated"));}catch(e){console.error("Error updating share:",e),this.showNotification(e.message||"Error updating share","error");}this.closeShareDialog(),await this.loadItems(),this.filterAndSortItems();},async removeSharedItem(){if(this.currentItem){try{let e=await fetch(`/api/shares/${this.currentItem.id}`,{method:"DELETE",headers:this._headers()});if(!e.ok&&204!==e.status)throw Error(`Server error ${e.status}`);this.showNotification(this.translate("shared_itemRemoved","Share removed"));}catch(e){console.error("Error removing share:",e),this.showNotification("Error removing share","error");}this.closeShareDialog(),await this.loadItems(),this.filterAndSortItems();}},sendNotification(){if(!this.currentItem)return;let e=document.getElementById("sv-notification-email"),t=document.getElementById("sv-notification-message"),i=e?e.value.trim():"",a=t?t.value.trim():"";i&&this.validateEmail(i)?window.fileSharing&&window.fileSharing.sendShareNotification&&window.fileSharing.sendShareNotification(this.currentItem.url,i,a).then(()=>{this.closeNotificationDialog(),this.showNotification(this.translate("shared_notificationSent","Notification sent"));}).catch(()=>this.showNotification(this.translate("shared_notificationFailed","Failed to send notification"),"error")):this.showNotification(this.translate("shared_invalidEmail","Please enter a valid email address"),"error");},showNotification(e,t="success"){window.ui&&window.ui.showNotification?window.ui.showNotification(e,t):alert(e);},validateEmail:e=>/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(e),formatDate:e=>window.formatDateShort?window.formatDateShort(e):String(e),translate:(e,t)=>window.i18n&&window.i18n.t?window.i18n.t(e,t):t};window.sharedView=e;