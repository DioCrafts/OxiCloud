#!/bin/bash
# Script para ejecutar el reparador de código Rust

# Colores para la salida
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🔧 Reparador Automático de Código Rust${NC}"

# Verificar si se proporcionó un directorio
if [ -n "$1" ]; then
    PROJECT_DIR=$1
    export PROJECT_DIR=$PROJECT_DIR
    echo -e "Reparando proyecto en: ${GREEN}$PROJECT_DIR${NC}"
else
    echo -e "Reparando proyecto en el directorio actual"
    export PROJECT_DIR="."
fi

# Verificar API key
if [ -n "$ANTHROPIC_API_KEY" ]; then
    echo -e "${GREEN}API key de Claude detectada${NC}"
else
    echo -e "${YELLOW}Advertencia: No se detectó API key de Claude. Se ejecutará sin asistencia avanzada.${NC}"
    echo -e "Para habilitar asistencia avanzada: export ANTHROPIC_API_KEY=tu_clave_api"
fi

# Preguntar si se quiere habilitar la auto-reparación
read -p "¿Activar reparación automática de código? (s/n) [s]: " AUTO_FIX
AUTO_FIX=${AUTO_FIX:-s}
if [[ $AUTO_FIX =~ ^[Ss]$ ]]; then
    export AUTO_FIX=True
    echo -e "${GREEN}Reparación automática activada${NC}"
else
    export AUTO_FIX=False
    echo -e "${YELLOW}Reparación automática desactivada - solo se generará análisis${NC}"
fi

# Ejecutar el reparador de código
python3 $(basename $0 .sh).py

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Proceso completado. Revisa rust_repair.log para más detalles.${NC}"
else
    echo -e "${RED}Error al ejecutar el reparador.${NC}"
fi
