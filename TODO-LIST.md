# OxiCloud TODO List

Este documento contiene la lista de tareas para el desarrollo de OxiCloud, un sistema de almacenamiento en la nube minimalista y eficiente similar a NextCloud pero optimizado para rendimiento.

## Fase 1: Funcionalidades básicas de archivos

### Sistema de carpetas
- [ ] Implementar API para crear carpetas
- [ ] Añadir soporte de rutas jerárquicas en el backend
- [ ] Actualizar UI para mostrar estructura de carpetas (árbol)
- [ ] Implementar navegación entre carpetas
- [ ] Añadir funcionalidad para renombrar carpetas
- [ ] Agregar opción de mover archivos entre carpetas

### Previsualización de archivos
- [ ] Implementar visor de imágenes integrado
- [ ] Añadir visor de PDF básico
- [ ] Generar miniaturas para imágenes
- [ ] Implementar iconos específicos según tipo de archivo
- [ ] Añadir vista previa de texto/código

### Buscador mejorado
- [ ] Implementar búsqueda por nombre
- [ ] Añadir filtros por tipo de archivo
- [ ] Implementar búsqueda por rango de fechas
- [ ] Agregar filtro por tamaño de archivo
- [ ] Añadir búsqueda dentro de carpetas específicas
- [ ] Implementar caché para resultados de búsqueda

### Optimizaciones UI/UX
- [ ] Mejorar diseño responsive para móviles
- [ ] Implementar drag & drop entre carpetas
- [ ] Añadir soporte para selección múltiple de archivos
- [ ] Implementar subida de archivos múltiples
- [ ] Añadir indicadores de progreso para operaciones largas
- [ ] Implementar notificaciones en UI para eventos

## Fase 2: Autenticación y multiusuario

### Sistema de usuarios
- [ ] Diseñar modelo de datos para usuarios
- [ ] Implementar registro de usuarios
- [ ] Crear sistema de inicio de sesión
- [ ] Añadir página de perfil de usuario
- [ ] Implementar recuperación de contraseña
- [ ] Separar almacenamiento por usuario

### Cuotas y permisos
- [ ] Implementar sistema de cuotas de almacenamiento
- [ ] Añadir sistema básico de roles (admin/usuario)
- [ ] Crear panel de administración
- [ ] Implementar permisos a nivel de carpeta
- [ ] Añadir monitoreo de uso de almacenamiento

### Seguridad básica
- [ ] Implementar hashing seguro de contraseñas con Argon2
- [ ] Añadir gestión de sesiones
- [ ] Implementar token de autenticación JWT
- [ ] Añadir protección CSRF
- [ ] Implementar límites de intentos de inicio de sesión
- [ ] Crear sistema de registro de actividad (logs)

## Fase 3: Características de colaboración

### Compartir archivos
- [ ] Implementar generación de enlaces compartidos
- [ ] Añadir configuración de permisos para enlaces
- [ ] Implementar protección con contraseña para enlaces
- [ ] Añadir fechas de expiración para enlaces compartidos
- [ ] Crear página para gestionar todos los recursos compartidos
- [ ] Implementar notificaciones al compartir

### Papelera de reciclaje
- [ ] Diseñar modelo para almacenar archivos eliminados
- [ ] Implementar eliminación soft (mover a papelera)
- [ ] Añadir funcionalidad para restaurar archivos
- [ ] Implementar purga automática por tiempo
- [ ] Añadir opción de vaciar papelera manualmente
- [ ] Implementar límites de almacenamiento para papelera

### Registro de actividad
- [ ] Crear modelo para eventos de actividad
- [ ] Implementar registro de operaciones CRUD
- [ ] Añadir registro de accesos y eventos de seguridad
- [ ] Crear página de historial de actividad
- [ ] Implementar filtros para el registro de actividad
- [ ] Añadir exportación de registro

## Fase 4: API y sincronización

### API REST completa
- [ ] Diseñar especificación OpenAPI
- [ ] Implementar endpoints para operaciones de archivos
- [ ] Añadir endpoints para usuarios y autenticación
- [ ] Implementar documentación automática (Swagger)
- [ ] Crear sistema de tokens de API
- [ ] Implementar limitación de tasa (rate limiting)
- [ ] Añadir versionado de API

### Soporte WebDAV
- [ ] Implementar servidor WebDAV básico
- [ ] Añadir autenticación para WebDAV
- [ ] Implementar operaciones PROPFIND
- [ ] Añadir soporte para bloqueo (locking)
- [ ] Probar compatibilidad con clientes estándar
- [ ] Optimizar rendimiento WebDAV

### Cliente de sincronización
- [ ] Diseñar arquitectura de cliente en Rust
- [ ] Implementar sincronización unidireccional
- [ ] Añadir sincronización bidireccional
- [ ] Implementar detección de conflictos
- [ ] Añadir opciones de configuración
- [ ] Crear versión mínima de cliente para Windows/macOS/Linux

## Fase 5: Funcionalidades avanzadas

### Cifrado de archivos
- [ ] Investigar y seleccionar algoritmos de cifrado
- [ ] Implementar cifrado en reposo para archivos
- [ ] Añadir gestión de claves
- [ ] Implementar cifrado para archivos compartidos
- [ ] Crear documentación de seguridad

### Versionado de archivos
- [ ] Diseñar sistema de almacenamiento de versiones
- [ ] Implementar historial de versiones
- [ ] Añadir visualización de diferencias
- [ ] Implementar restauración de versiones
- [ ] Añadir políticas de retención de versiones

### Aplicaciones básicas
- [ ] Diseñar sistema de plugins/apps
- [ ] Implementar visor/editor de texto básico
- [ ] Añadir aplicación de notas simple
- [ ] Implementar calendario básico
- [ ] Crear API para aplicaciones de terceros

## Optimizaciones continuas

### Backend
- [ ] Implementar caché de archivos con Rust
- [ ] Optimizar transmisión de archivos grandes
- [ ] Añadir compresión adaptativa según tipo de archivo
- [ ] Implementar procesamiento asíncrono para tareas pesadas
- [ ] Optimizar consultas a base de datos
- [ ] Implementar estrategias de escalado

### Frontend
- [ ] Optimizar carga inicial de assets
- [ ] Implementar lazy loading para listas grandes
- [ ] Añadir caché local (localStorage/IndexedDB)
- [ ] Optimizar renderizado de UI
- [ ] Implementar precarga inteligente (prefetching)
- [ ] Añadir soporte offline básico

### Almacenamiento
- [ ] Investigar opciones de deduplicación
- [ ] Implementar almacenamiento por bloques
- [ ] Añadir compresión transparente según tipo de archivo
- [ ] Implementar rotación y archivado de logs
- [ ] Crear sistema de respaldo automatizado
- [ ] Añadir soporte para almacenamiento distribuido

## Infraestructura y despliegue

- [ ] Crear configuración para Docker
- [ ] Implementar CI/CD con GitHub Actions
- [ ] Añadir pruebas automatizadas
- [ ] Crear documentación de instalación
- [ ] Implementar monitoreo y alertas
- [ ] Añadir sistema de actualizaciones automáticas
