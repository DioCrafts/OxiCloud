-- Migración para el sistema de compartición de archivos

-- Tabla para almacenar permisos de compartición entre usuarios
CREATE TABLE IF NOT EXISTS auth.shared_files (
    id SERIAL PRIMARY KEY,
    file_id VARCHAR(255) NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    permission_level VARCHAR(50) NOT NULL, -- 'read', 'write', 'admin'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_file_user_share UNIQUE (file_id, user_id)
);

-- Tabla para enlaces públicos de compartición
CREATE TABLE IF NOT EXISTS auth.public_links (
    id VARCHAR(64) PRIMARY KEY, -- ID único para la URL
    file_id VARCHAR(255) NOT NULL,
    owner_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    permission_level VARCHAR(50) NOT NULL, -- 'read', 'write', 'admin'
    password_hash VARCHAR(255) NULL, -- Hash de la contraseña (NULL si no hay contraseña)
    expires_at TIMESTAMPTZ NULL, -- NULL significa que no expira
    access_count INT NOT NULL DEFAULT 0, -- Contador de accesos
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Índices para optimizar búsquedas frecuentes
CREATE INDEX IF NOT EXISTS idx_shared_files_file_id ON auth.shared_files(file_id);
CREATE INDEX IF NOT EXISTS idx_shared_files_user_id ON auth.shared_files(user_id);
CREATE INDEX IF NOT EXISTS idx_public_links_file_id ON auth.public_links(file_id);
CREATE INDEX IF NOT EXISTS idx_public_links_owner_id ON auth.public_links(owner_id);

COMMENT ON TABLE auth.shared_files IS 'Almacena la información de archivos compartidos entre usuarios';
COMMENT ON TABLE auth.public_links IS 'Almacena la información de enlaces públicos para compartir archivos';
COMMENT ON COLUMN auth.shared_files.permission_level IS 'Nivel de permiso: read (solo lectura), write (lectura/escritura), admin (lectura/escritura/eliminar)';
COMMENT ON COLUMN auth.public_links.permission_level IS 'Nivel de permiso: read (solo lectura), write (lectura/escritura), admin (lectura/escritura/eliminar)';
COMMENT ON COLUMN auth.public_links.password_hash IS 'Hash de la contraseña opcional para proteger el enlace. NULL significa sin contraseña';
COMMENT ON COLUMN auth.public_links.expires_at IS 'Fecha y hora de caducidad del enlace. NULL significa que no caduca';