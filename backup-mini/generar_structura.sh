#!/bin/bash

# Crear directorios
mkdir -p oxicloud/src/api
mkdir -p oxicloud/src/core
mkdir -p oxicloud/src/storage
mkdir -p oxicloud/src/ui
mkdir -p oxicloud/src/utils
mkdir -p oxicloud/templates
mkdir -p oxicloud/static/css
mkdir -p oxicloud/static/js
mkdir -p oxicloud/static/images
mkdir -p oxicloud/migrations

# Crear archivos en la raíz
touch oxicloud/Cargo.toml

# Crear archivos en src/
touch oxicloud/src/main.rs

# Archivos en src/api/
touch oxicloud/src/api/mod.rs
touch oxicloud/src/api/auth.rs
touch oxicloud/src/api/files.rs
touch oxicloud/src/api/users.rs

# Archivos en src/core/
touch oxicloud/src/core/mod.rs
touch oxicloud/src/core/config.rs
touch oxicloud/src/core/db.rs
touch oxicloud/src/core/files.rs
touch oxicloud/src/core/users.rs

# Archivos en src/storage/
touch oxicloud/src/storage/mod.rs
touch oxicloud/src/storage/disk.rs
touch oxicloud/src/storage/models.rs

# Archivos en src/ui/
touch oxicloud/src/ui/mod.rs
touch oxicloud/src/ui/templates.rs
touch oxicloud/src/ui/assets.rs

# Archivos en src/utils/
touch oxicloud/src/utils/mod.rs
touch oxicloud/src/utils/errors.rs
touch oxicloud/src/utils/logging.rs

# Archivos en templates/
touch oxicloud/templates/login.html
touch oxicloud/templates/files.html
touch oxicloud/templates/layout.html

echo "Estructura de carpetas y ficheros creada correctamente."

