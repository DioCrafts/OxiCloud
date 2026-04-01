# Configuración para Docker Hub y GitHub Actions

Este documento explica cómo configurar los secretos necesarios para publicar imágenes de Docker en Docker Hub usando GitHub Actions.

## Requisitos previos

1. Una cuenta en [Docker Hub](https://hub.docker.com/)
2. Un repositorio en Docker Hub donde subir la imagen
3. Un token de acceso personal (PAT) de Docker Hub

## Pasos para configurar los secretos en GitHub

1. Genera un token de acceso en Docker Hub
   - Inicia sesión en [Docker Hub](https://hub.docker.com/)
   - Ve a tu perfil (esquina superior derecha) → Account Settings → Security
   - Haz clic en "New Access Token"
   - Proporciona una descripción como "GitHub Actions"
   - Selecciona los permisos apropiados (normalmente "Read, Write, Delete")
   - Haz clic en "Generate"
   - **IMPORTANTE**: Copia el token generado, ya que no podrás verlo de nuevo

2. Configura los secretos en tu repositorio de GitHub
   - Ve a tu repositorio en GitHub
   - Haz clic en "Settings" → "Secrets and variables" → "Actions"
   - Haz clic en "New repository secret"
   - Añade los siguientes secretos:
     - Nombre: `DOCKERHUB_USERNAME` 
       Valor: Tu nombre de usuario de Docker Hub
     - Nombre: `DOCKERHUB_TOKEN`
       Valor: El token de acceso que generaste en el paso anterior

## Uso

Una vez configurados los secretos, los flujos de trabajo de GitHub Actions podrán autenticarse con Docker Hub y publicar imágenes. 

Cuando crees una nueva [release en GitHub](https://docs.github.com/es/repositories/releasing-projects-on-github/managing-releases-in-a-repository#creating-a-release), el flujo de trabajo `docker-publish.yml` se activará automáticamente y:

1. Construirá la imagen de Docker
2. La etiquetará con el número de versión de la release
3. La subirá a Docker Hub

## Verificación

Para verificar que la configuración está correcta:

1. Crea una nueva release en GitHub
2. Ve a la pestaña "Actions" y observa el progreso del flujo de trabajo
3. Una vez completado, verifica que la imagen aparezca en tu repositorio de Docker Hub

## Notas adicionales

- Para entornos de producción, considera usar un usuario de servicio en Docker Hub en lugar de tu cuenta personal
- Rota regularmente los tokens de acceso para mayor seguridad
- Considera agregar escaneo de vulnerabilidades en las imágenes como parte del flujo de trabajo