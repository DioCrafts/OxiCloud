let e={_icon:(e,t="")=>window.oxiIcon?window.oxiIcon(e,t):"",async searchFiles(e,t={}){try{let r=new URLSearchParams;r.append("query",e),t.folder_id&&r.append("folder_id",t.folder_id),void 0!==t.recursive&&r.append("recursive",t.recursive),t.file_types&&r.append("type",t.file_types),t.min_size&&r.append("min_size",t.min_size),t.max_size&&r.append("max_size",t.max_size),t.created_after&&r.append("created_after",t.created_after),t.created_before&&r.append("created_before",t.created_before),t.modified_after&&r.append("modified_after",t.modified_after),t.modified_before&&r.append("modified_before",t.modified_before),t.limit&&r.append("limit",t.limit),t.offset&&r.append("offset",t.offset),t.sort_by&&r.append("sort_by",t.sort_by);let i=`/api/search?${r.toString()}`;console.log(`[search] GET ${i}`);let s=await fetch(i,{headers:getAuthHeaders()});if(s.ok)return await s.json();{let e="";try{e=(await s.json()).error||s.statusText;}catch(t){e=s.statusText;}throw console.error(`Search error: ${e}`),Error(`Search failed: ${e}`);}}catch(e){return console.error("Error performing search:",e),window.ui.showNotification("Error","Error performing search"),{files:[],folders:[],total_count:0,query_time_ms:0,sort_by:"relevance"};}},async getSuggestions(e,t={}){try{let r=new URLSearchParams;r.append("query",e),t.folder_id&&r.append("folder_id",t.folder_id),t.limit&&r.append("limit",t.limit);let i=`/api/search/suggest?${r.toString()}`,s=await fetch(i,{headers:getAuthHeaders()});if(s.ok)return await s.json();return{suggestions:[],query_time_ms:0};}catch(e){return console.error("Error getting suggestions:",e),{suggestions:[],query_time_ms:0};}},displaySearchResults(e){let t=document.getElementById("files-grid"),r=document.getElementById("files-list-view");t.innerHTML="",r.innerHTML=`
            <div class="list-header">
                <div class="list-header-checkbox"><input type="checkbox" id="select-all-checkbox" title="Select all"></div>
                <div data-i18n="files.name">Name</div>
                <div data-i18n="files.type">Type</div>
                <div data-i18n="files.size">Size</div>
                <div data-i18n="files.modified">Modified</div>
            </div>
        `;let i=e.total_count||e.files.length+e.folders.length,s=void 0!==e.query_time_ms?` <span class="search-time">(${e.query_time_ms}ms)</span>`:"",a=document.createElement("div");a.className="search-results-header",a.innerHTML=`
            <h3>Search results (${i})${s}</h3>
            <div class="search-controls">
                <select id="search-sort-select" class="search-sort-select" title="Sort by">
                    <option value="relevance"${"relevance"===e.sort_by?" selected":""}>Relevance</option>
                    <option value="name"${"name"===e.sort_by?" selected":""}>Name A-Z</option>
                    <option value="name_desc"${"name_desc"===e.sort_by?" selected":""}>Name Z-A</option>
                    <option value="date_desc"${"date_desc"===e.sort_by?" selected":""}>Newest first</option>
                    <option value="date"${"date"===e.sort_by?" selected":""}>Oldest first</option>
                    <option value="size_desc"${"size_desc"===e.sort_by?" selected":""}>Largest first</option>
                    <option value="size"${"size"===e.sort_by?" selected":""}>Smallest first</option>
                </select>
                <button class="btn btn-secondary" id="clear-search-btn">
                    ${this._icon("times")} Clear search
                </button>
            </div>
        `,t.appendChild(a);let o=document.getElementById("search-sort-select");o&&o.addEventListener("change",()=>{let e=document.querySelector(".search-container input");if(e&&e.value.trim()){let e=new CustomEvent("search-resort",{detail:{sort_by:o.value}});document.dispatchEvent(e);}});let n=document.getElementById("clear-search-btn");if(n&&n.addEventListener("click",()=>{document.querySelector(".search-container input").value="",window.app.currentPath="",window.app.isSearchMode=!1,window.ui.updateBreadcrumb(""),window.loadFiles();}),0===e.files.length&&0===e.folders.length){let e=document.createElement("div");e.className="empty-state",e.innerHTML=`
                ${this._icon("search")}
                <p style="color: var(--text-secondary, #64748b);">No results found for this search</p>
            `,t.appendChild(e);return;}window.ui.renderFolders(e.folders||[]),window.ui.renderFiles(e.files||[]);},async clearSearchCache(){try{if((await fetch("/api/search/cache",{method:"DELETE",headers:getAuthHeaders()})).ok)return window.ui.showNotification("Cache cleared","Search cache cleared successfully"),!0;return window.ui.showNotification("Error","Error clearing search cache"),!1;}catch(e){return console.error("Error clearing search cache:",e),window.ui.showNotification("Error","Error clearing search cache"),!1;}}};window.search=e;