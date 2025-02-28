#!/usr/bin/env python3
import os
import glob
import re
import json
import time
import asyncio
import aiohttp
import subprocess
import logging
import random
import shutil
from pathlib import Path
from typing import List, Dict, Optional, Tuple, Any, Set

class EnhancedRustRepairAgent:
    def __init__(self, api_key: str, project_dir: str, max_concurrency: int = 2, auto_fix: bool = True):
        self.api_key = api_key
        self.project_dir = Path(project_dir)
        self.logger = self._setup_logger()
        self.semaphore = asyncio.Semaphore(max_concurrency)
        self.auto_fix = auto_fix  # Si es True, aplica correcciones automáticamente
        self.stats = {
            "total_files": 0,
            "processed_files": 0,
            "fixed_files": 0,
            "fixed_errors": 0,
            "compiled_successfully": False,
            "start_time": None,
            "end_time": None,
            "progress_file": "repair_progress.json"
        }
        self._load_progress()
        self.error_patterns = self._compile_error_patterns()
        self.advanced_error_patterns = self._compile_advanced_error_patterns()
        
    def _setup_logger(self) -> logging.Logger:
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler('rust_repair.log'),
                logging.StreamHandler()
            ]
        )
        return logging.getLogger(__name__)
    
    def _compile_error_patterns(self) -> Dict[str, Dict]:
        """Compila patrones de errores comunes de Rust y sus soluciones"""
        return {
            "no_targets": {
                "pattern": r"no targets specified in the manifest.*src/lib.rs, src/main.rs",
                "description": "No hay archivo principal o biblioteca especificado",
                "fix_function": self._fix_no_targets
            },
            "missing_dependency": {
                "pattern": r"failed to resolve: use of undeclared (crate|type) or module `([^`]+)`",
                "description": "Dependencia no declarada o módulo faltante",
                "fix_function": self._fix_missing_dependency
            },
            "module_not_found": {
                "pattern": r"module `([^`]+)` is private",
                "description": "Módulo privado que debe ser público",
                "fix_function": self._fix_module_privacy
            },
            "missing_field": {
                "pattern": r"missing field `([^`]+)` in initializer of `([^`]+)`",
                "description": "Campo faltante en inicializador de estructura",
                "fix_function": self._fix_missing_field
            },
            "use_of_undeclared_variable": {
                "pattern": r"cannot find value `([^`]+)` in this scope",
                "description": "Variable no declarada",
                "fix_function": self._fix_undeclared_variable
            }
        }
    
    def _compile_advanced_error_patterns(self) -> Dict[str, Dict]:
        """Compila patrones para errores más específicos relacionados con proyectos grandes migrados"""
        return {
            "invalid_module_name": {
                "pattern": r"invalid character `[@\.\-]` in crate name|expected identifier, found",
                "description": "Nombre de módulo inválido (contiene caracteres no permitidos)",
                "fix_function": self._fix_invalid_module_names,
                "scan_function": self._scan_for_invalid_module_names
            },
            "duplicate_module_definitions": {
                "pattern": r"the name `([^`]+)` is defined multiple times",
                "description": "Definiciones duplicadas de módulos",
                "fix_function": self._fix_duplicate_modules,
                "scan_function": self._scan_for_duplicate_modules
            },
            "unclosed_delimiter": {
                "pattern": r"this (.*?) starts here.*?but is never closed",
                "description": "Delimitador no cerrado",
                "fix_function": self._fix_unclosed_delimiters,
                "scan_function": self._scan_for_unclosed_delimiters
            },
            "missing_mod_declaration": {
                "pattern": r"file not included in module tree",
                "description": "Archivo no incluido en el árbol de módulos",
                "fix_function": self._fix_missing_mod_declarations,
                "scan_function": self._scan_for_missing_mod_declarations
            }
        }
    
    def _load_progress(self):
        """Carga el progreso guardado de reparación"""
        progress_file = Path(self.stats["progress_file"])
        if progress_file.exists():
            try:
                with open(progress_file, 'r') as f:
                    saved_stats = json.load(f)
                    self.logger.info(f"Cargando progreso previo: {saved_stats['processed_files']} archivos procesados")
                    self.stats.update(saved_stats)
            except Exception as e:
                self.logger.error(f"Error al cargar progreso: {str(e)}")
    
    def _save_progress(self):
        """Guarda el progreso actual de reparación"""
        try:
            with open(self.stats["progress_file"], 'w') as f:
                json.dump(self.stats, f)
            self.logger.info(f"Progreso guardado: {self.stats['processed_files']}/{self.stats['total_files']} archivos, {self.stats['fixed_files']} reparados")
        except Exception as e:
            self.logger.error(f"Error al guardar progreso: {str(e)}")
    
    async def repair_codebase(self):
        """Proceso principal para reparar la base de código Rust"""
        self.stats["start_time"] = time.time()
        self.logger.info("Iniciando reparación de código Rust...")
        
        # Analizar el proyecto
        self._analyze_project()
        
        # Realizar un escaneo proactivo de errores conocidos (antes de compilar)
        await self._proactive_scan()
        
        # Intentar la primera compilación para identificar errores
        success, errors = self._build_project()
        
        # Ciclo de reparación
        iteration = 0
        max_iterations = 15  # Aumentado para proyectos complejos
        
        while not success and iteration < max_iterations:
            iteration += 1
            self.logger.info(f"Iniciando ciclo de reparación {iteration}/{max_iterations}")
            
            # Analizar y clasificar errores
            error_infos = self._parse_build_errors(errors)
            
            if error_infos:
                self.logger.info(f"Encontrados {len(error_infos)} errores estándar para reparar")
                # Reparar errores identificados
                fixed_count = await self._fix_errors(error_infos)
                
                if fixed_count > 0:
                    self.logger.info(f"Se corrigieron {fixed_count} errores en este ciclo")
                    # Recompilar para verificar si los errores se solucionaron
                    success, errors = self._build_project()
                    if success:
                        break
                    # Continuar al siguiente ciclo si se hicieron correcciones
                    continue
            
            # Si no se pudieron identificar errores estándar o no se pudieron corregir,
            # buscar errores avanzados específicos del proyecto
            advanced_fixes = await self._scan_and_fix_advanced_errors(errors)
            
            if not advanced_fixes:
                # Si no se pudieron aplicar correcciones avanzadas, solicitar ayuda a Claude
                self.logger.warning("No se pudieron identificar o corregir errores específicos, solicitando asistencia a Claude")
                if self.api_key:
                    await self._get_claude_assistance(errors)
                else:
                    self.logger.warning("No se proporcionó API_KEY, omitiendo asistencia de Claude")
            
            # Recompilar para verificar si los errores se solucionaron
            success, errors = self._build_project()
            
            if success:
                self.logger.info("¡Compilación exitosa! Todos los errores fueron reparados.")
                self.stats["compiled_successfully"] = True
                break
        
        # Si no se pudo compilar después de todos los intentos
        if not success:
            self.logger.warning("No se pudo reparar completamente el código después de varios intentos")
            self.logger.info("Generando reporte de análisis de código")
            if self.api_key:
                await self._generate_analysis_report(errors)
            else:
                self._generate_simple_analysis_report(errors)
        
        self.stats["end_time"] = time.time()
        duration = self.stats["end_time"] - self.stats["start_time"]
        
        self.logger.info(f"Reparación completada en {duration:.2f} segundos")
        self.logger.info(f"Total de archivos: {self.stats['total_files']}")
        self.logger.info(f"Archivos procesados: {self.stats['processed_files']}")
        self.logger.info(f"Archivos reparados: {self.stats['fixed_files']}")
        self.logger.info(f"Errores corregidos: {self.stats['fixed_errors']}")
        self.logger.info(f"Compilación exitosa: {self.stats['compiled_successfully']}")
        
        self._save_progress()
    
    def _analyze_project(self):
        """Analiza la estructura del proyecto Rust"""
        self.logger.info(f"Analizando proyecto en {self.project_dir}")
        
        # Verificar Cargo.toml
        cargo_toml = self.project_dir / "Cargo.toml"
        if not cargo_toml.exists():
            self.logger.error("No se encontró Cargo.toml. No es un proyecto Rust válido.")
            raise FileNotFoundError("No se encontró Cargo.toml")
        
        # Contar archivos Rust
        rust_files = list(self.project_dir.rglob("**/*.rs"))
        self.stats["total_files"] = len(rust_files)
        
        # Verificar estructura de directorios
        src_dir = self.project_dir / "src"
        if not src_dir.exists():
            self.logger.warning("No existe directorio src/, creándolo")
            src_dir.mkdir(exist_ok=True)
        
        self.logger.info(f"Encontrados {self.stats['total_files']} archivos Rust")
    
    async def _proactive_scan(self):
        """Realiza un escaneo proactivo de errores conocidos en el proyecto"""
        self.logger.info("Realizando escaneo proactivo de problemas conocidos...")
        
        # Ejecutar cada función de escaneo avanzado
        for error_type, error_config in self.advanced_error_patterns.items():
            if "scan_function" in error_config:
                scan_function = error_config["scan_function"]
                try:
                    issues = await scan_function()
                    if issues:
                        self.logger.info(f"Encontrados {len(issues)} problemas de tipo '{error_type}'")
                        
                        if self.auto_fix and "fix_function" in error_config:
                            fix_function = error_config["fix_function"]
                            for issue in issues:
                                try:
                                    if await fix_function(issue):
                                        self.stats["fixed_errors"] += 1
                                except Exception as e:
                                    self.logger.error(f"Error al corregir {error_type}: {str(e)}")
                
                except Exception as e:
                    self.logger.error(f"Error durante el escaneo de {error_type}: {str(e)}")
    
    async def _scan_for_invalid_module_names(self) -> List[Dict]:
        """Busca nombres de módulos inválidos en el proyecto"""
        self.logger.info("Buscando nombres de módulos inválidos...")
        issues = []
        
        # Buscar archivos mod.rs para encontrar declaraciones de módulos
        for mod_file in self.project_dir.rglob("**/mod.rs"):
            try:
                content = mod_file.read_text(encoding='utf-8', errors='replace')
                # Buscar declaraciones de módulos con caracteres inválidos
                matches = re.finditer(r'(?:pub\s+)?mod\s+([a-zA-Z0-9_@\.\-]+)\s*;', content)
                
                for match in matches:
                    module_name = match.group(1)
                    if re.search(r'[@\.\-]', module_name):
                        issues.append({
                            "type": "invalid_module_name",
                            "file": str(mod_file),
                            "module_name": module_name,
                            "line_match": match.group(0),
                            "content": content
                        })
            except Exception as e:
                self.logger.error(f"Error al escanear {mod_file}: {str(e)}")
        
        return issues
    
    async def _fix_invalid_module_names(self, issue: Dict) -> bool:
        """Corrige nombres de módulos inválidos"""
        file_path = issue["file"]
        module_name = issue["module_name"]
        content = issue["content"]
        line_match = issue["line_match"]
        
        self.logger.info(f"Corrigiendo nombre de módulo inválido: {module_name} en {file_path}")
        
        # Crear nombre de módulo válido
        valid_name = re.sub(r'@', '_at_', module_name)
        valid_name = re.sub(r'\.', '_dot_', valid_name)
        valid_name = re.sub(r'-', '_dash_', valid_name)
        
        # Reemplazar la declaración del módulo
        new_line = line_match.replace(module_name, valid_name)
        new_content = content.replace(line_match, new_line)
        
        # Guardar cambios
        Path(file_path).write_text(new_content)
        
        # Verificar si existe el archivo correspondiente y renombrarlo
        parent_dir = Path(file_path).parent
        old_module_file = parent_dir / f"{module_name}.rs"
        new_module_file = parent_dir / f"{valid_name}.rs"
        
        if old_module_file.exists():
            self.logger.info(f"Renombrando archivo {old_module_file} a {new_module_file}")
            try:
                shutil.move(old_module_file, new_module_file)
            except Exception as e:
                self.logger.error(f"Error al renombrar archivo: {str(e)}")
        
        self.stats["fixed_files"] += 1
        return True
    
    async def _scan_for_duplicate_modules(self) -> List[Dict]:
        """Busca definiciones duplicadas de módulos (archivo .rs y directorio con mod.rs)"""
        self.logger.info("Buscando definiciones duplicadas de módulos...")
        issues = []
        
        src_dir = self.project_dir / "src"
        
        # Buscar todos los directorios que contienen mod.rs
        for mod_file in src_dir.rglob("**/mod.rs"):
            dir_path = mod_file.parent
            dir_name = dir_path.name
            parent_dir = dir_path.parent
            
            # Verificar si existe un archivo .rs con el mismo nombre en el directorio padre
            file_module = parent_dir / f"{dir_name}.rs"
            
            if file_module.exists():
                issues.append({
                    "type": "duplicate_module",
                    "dir_module": str(mod_file),
                    "file_module": str(file_module),
                    "module_name": dir_name,
                    "parent_dir": str(parent_dir)
                })
        
        return issues
    
    async def _fix_duplicate_modules(self, issue: Dict) -> bool:
        """Corrige definiciones duplicadas de módulos"""
        dir_module = issue["dir_module"]
        file_module = issue["file_module"]
        module_name = issue["module_name"]
        
        self.logger.info(f"Corrigiendo definición duplicada para módulo '{module_name}'")
        
        # Estrategia: eliminar el archivo .rs y mantener solo el directorio con mod.rs
        try:
            # Comprobar si hay contenido importante en el archivo .rs
            file_content = Path(file_module).read_text(encoding='utf-8', errors='replace')
            mod_content = Path(dir_module).read_text(encoding='utf-8', errors='replace')
            
            # Si el archivo .rs tiene contenido que no está en mod.rs, fusionarlos
            if len(file_content.strip()) > 0 and file_content.strip() not in mod_content:
                self.logger.info(f"Fusionando contenido de {file_module} en {dir_module}")
                with open(dir_module, 'a') as f:
                    f.write(f"\n// Contenido fusionado desde {file_module}\n")
                    f.write(file_content)
            
            # Eliminar el archivo .rs
            self.logger.info(f"Eliminando archivo duplicado: {file_module}")
            Path(file_module).unlink()
            
            self.stats["fixed_files"] += 1
            return True
            
        except Exception as e:
            self.logger.error(f"Error al corregir módulo duplicado: {str(e)}")
            return False
    
    async def _scan_for_unclosed_delimiters(self) -> List[Dict]:
        """Busca delimitadores no cerrados en los archivos"""
        self.logger.info("Buscando delimitadores no cerrados...")
        issues = []
        
        # Esta es una tarea compleja que requeriría un análisis sintáctico completo.
        # En su lugar, buscaremos patrones comunes de problemas.
        
        # Por ejemplo, funciones sin llave de cierre
        for rs_file in self.project_dir.rglob("**/*.rs"):
            try:
                content = rs_file.read_text(encoding='utf-8', errors='replace')
                
                # Contar llaves abiertas y cerradas
                open_braces = content.count('{')
                close_braces = content.count('}')
                
                if open_braces > close_braces:
                    # Tenemos delimitadores no cerrados
                    issues.append({
                        "type": "unclosed_delimiter",
                        "file": str(rs_file),
                        "missing_count": open_braces - close_braces,
                        "content": content
                    })
            except Exception as e:
                self.logger.error(f"Error al escanear {rs_file}: {str(e)}")
        
        return issues
    
    async def _fix_unclosed_delimiters(self, issue: Dict) -> bool:
        """Intenta corregir delimitadores no cerrados"""
        file_path = issue["file"]
        content = issue["content"]
        missing_count = issue["missing_count"]
        
        self.logger.info(f"Intentando corregir {missing_count} delimitadores no cerrados en {file_path}")
        
        # Este es un enfoque básico y puede no funcionar en todos los casos
        lines = content.split('\n')
        modified = False
        
        # Estrategia 1: Buscar líneas que terminan con "-> Result<..." sin llave de apertura
        for i, line in enumerate(lines):
            if re.search(r'-> .*Result<.*>$', line) and i + 1 < len(lines) and '{' not in lines[i + 1]:
                lines[i + 1] = lines[i + 1] + " {"
                modified = True
        
        # Estrategia 2: Agregar llaves faltantes al final del archivo
        if missing_count > 0:
            lines.append('\n' + '}' * missing_count + " // Añadido por reparador automático")
            modified = True
        
        if modified:
            # Guardar el archivo modificado
            Path(file_path).write_text('\n'.join(lines))
            self.stats["fixed_files"] += 1
            return True
        
        return False
    
    async def _scan_for_missing_mod_declarations(self) -> List[Dict]:
        """Busca archivos .rs que no están declarados en ningún mod.rs"""
        self.logger.info("Buscando archivos no declarados en el árbol de módulos...")
        issues = []
        
        src_dir = self.project_dir / "src"
        
        # Encontrar todos los archivos .rs
        all_rs_files = set()
        for rs_file in src_dir.rglob("**/*.rs"):
            if rs_file.name != "mod.rs" and rs_file.name != "lib.rs" and rs_file.name != "main.rs":
                all_rs_files.add(rs_file)
        
        # Encontrar todas las declaraciones de módulos en archivos mod.rs
        declared_modules = set()
        for mod_file in src_dir.rglob("**/mod.rs"):
            parent_dir = mod_file.parent
            content = mod_file.read_text(encoding='utf-8', errors='replace')
            
            # Buscar declaraciones de módulos
            matches = re.finditer(r'(?:pub\s+)?mod\s+([a-zA-Z0-9_]+)\s*;', content)
            
            for match in matches:
                module_name = match.group(1)
                module_file = parent_dir / f"{module_name}.rs"
                if module_file.exists():
                    declared_modules.add(module_file)
        
        # Encontrar archivos no declarados
        undeclared_files = all_rs_files - declared_modules
        
        for file in undeclared_files:
            # Solo considerar archivos que estén dentro de directorios con mod.rs
            parent_dir = file.parent
            if (parent_dir / "mod.rs").exists():
                issues.append({
                    "type": "missing_mod_declaration",
                    "file": str(file),
                    "mod_file": str(parent_dir / "mod.rs"),
                    "module_name": file.stem
                })
        
        return issues
    
    async def _fix_missing_mod_declarations(self, issue: Dict) -> bool:
        """Añade declaraciones de módulos faltantes en los archivos mod.rs"""
        file_path = issue["file"]
        mod_file = issue["mod_file"]
        module_name = issue["module_name"]
        
        self.logger.info(f"Añadiendo declaración para módulo '{module_name}' en {mod_file}")
        
        try:
            # Añadir la declaración al final del archivo mod.rs
            with open(mod_file, 'a') as f:
                f.write(f"\npub mod {module_name}; // Añadido por reparador automático\n")
            
            self.stats["fixed_files"] += 1
            return True
            
        except Exception as e:
            self.logger.error(f"Error al añadir declaración de módulo: {str(e)}")
            return False
    
    def _build_project(self) -> Tuple[bool, str]:
        """Intenta compilar el proyecto y devuelve el resultado y errores"""
        self.logger.info("Compilando proyecto...")
        
        try:
            result = subprocess.run(
                ["cargo", "build"],
                cwd=self.project_dir,
                text=True,
                capture_output=True
            )
            
            # Verificar si la compilación fue exitosa
            if result.returncode == 0:
                self.logger.info("Compilación exitosa")
                return True, ""
            else:
                self.logger.warning("La compilación falló con errores")
                self.logger.debug(f"Error de compilación: {result.stderr}")
                return False, result.stderr
                
        except Exception as e:
            self.logger.error(f"Error al ejecutar cargo build: {str(e)}")
            return False, str(e)
    
    def _parse_build_errors(self, error_output: str) -> List[Dict]:
        """Analiza la salida de error de cargo build y clasifica los errores"""
        error_infos = []
        
        for error_type, error_config in self.error_patterns.items():
            matches = re.finditer(error_config["pattern"], error_output)
            
            for match in matches:
                error_info = {
                    "type": error_type,
                    "description": error_config["description"],
                    "match": match,
                    "line": self._extract_line_number(error_output, match),
                    "file": self._extract_file_path(error_output, match),
                }
                error_infos.append(error_info)
        
        return error_infos
    
    def _extract_line_number(self, error_output: str, match) -> Optional[int]:
        """Extrae el número de línea del error"""
        # Buscar líneas con números de línea antes del match
        context = error_output[:match.start()]
        line_match = re.search(r'-->.*:(\d+):\d+', context, re.MULTILINE)
        if line_match:
            return int(line_match.group(1))
        return None
    
    def _extract_file_path(self, error_output: str, match) -> Optional[str]:
        """Extrae la ruta del archivo con el error"""
        # Buscar ruta de archivo antes del match
        context = error_output[:match.start()]
        file_match = re.search(r'-->.*?([^:]+):\d+:\d+', context, re.MULTILINE)
        if file_match:
            return file_match.group(1).strip()
        return None
    
    async def _fix_errors(self, error_infos: List[Dict]) -> int:
        """Aplica las correcciones para los errores identificados"""
        fixed_count = 0
        
        for error_info in error_infos:
            error_type = error_info["type"]
            
            if error_type in self.error_patterns:
                fix_function = self.error_patterns[error_type]["fix_function"]
                try:
                    if await fix_function(error_info):
                        fixed_count += 1
                        self.stats["fixed_errors"] += 1
                except Exception as e:
                    self.logger.error(f"Error al aplicar corrección para {error_type}: {str(e)}")
        
        return fixed_count
    
    async def _scan_and_fix_advanced_errors(self, error_output: str) -> bool:
        """Busca y corrige errores avanzados en la salida de compilación"""
        fixed_any = False
        
        for error_type, error_config in self.advanced_error_patterns.items():
            if re.search(error_config["pattern"], error_output):
                self.logger.info(f"Detectado error avanzado: {error_config['description']}")
                
                if "scan_function" in error_config and "fix_function" in error_config:
                    scan_function = error_config["scan_function"]
                    fix_function = error_config["fix_function"]
                    
                    try:
                        issues = await scan_function()
                        if issues:
                            self.logger.info(f"Encontrados {len(issues)} problemas de tipo '{error_type}'")
                            
                            for issue in issues:
                                if await fix_function(issue):
                                    fixed_any = True
                                    self.stats["fixed_errors"] += 1
                    
                    except Exception as e:
                        self.logger.error(f"Error al procesar {error_type}: {str(e)}")
        
        return fixed_any
    
    async def _fix_no_targets(self, error_info: Dict) -> bool:
        """Soluciona el error de falta de targets en el proyecto"""
        self.logger.info("Reparando: No hay archivo principal o biblioteca especificado")
        
        src_dir = self.project_dir / "src"
        
        # Estrategia 1: Buscar un archivo index.rs y renombrarlo a main.rs
        index_rs = src_dir / "index.rs"
        if index_rs.exists():
            main_rs = src_dir / "main.rs"
            self.logger.info(f"Renombrando {index_rs} a {main_rs}")
            os.rename(index_rs, main_rs)
            return True
        
        # Estrategia 2: Crear un archivo main.rs básico
        if not (src_dir / "main.rs").exists():
            self.logger.info("Creando archivo main.rs básico")
            main_content = """fn main() {
    println!("Hello, OxiCloud!");
    // Inicialización de la aplicación
}
"""
            (src_dir / "main.rs").write_text(main_content)
            return True
        
        return False
    
    async def _fix_missing_dependency(self, error_info: Dict) -> bool:
        """Soluciona errores de dependencias o módulos faltantes"""
        match = error_info["match"]
        crate_name = match.group(2) if match.group(1) == "crate" else match.group(2)
        file_path = error_info["file"]
        
        if not file_path:
            return False
        
        self.logger.info(f"Reparando: Dependencia/módulo faltante '{crate_name}' en {file_path}")
        
        # Agregar use statement al inicio del archivo
        file_content = Path(file_path).read_text()
        
        # Evitar duplicar importaciones
        if f"use {crate_name};" not in file_content:
            modified_content = f"use {crate_name};\n\n{file_content}"
            Path(file_path).write_text(modified_content)
            return True
        
        return False
    
    async def _fix_module_privacy(self, error_info: Dict) -> bool:
        """Soluciona errores de privacidad de módulos"""
        match = error_info["match"]
        module_name = match.group(1)
        
        # Buscar archivos mod.rs o módulos en los directorios del proyecto
        src_dir = self.project_dir / "src"
        mod_files = list(src_dir.rglob("mod.rs"))
        
        for mod_file in mod_files:
            # Verificar si este mod.rs contiene el módulo privado
            content = mod_file.read_text()
            
            # Buscar el módulo sin pub
            mod_pattern = re.compile(fr"mod\s+{module_name}\s*;")
            if mod_pattern.search(content):
                # Reemplazar con pub mod
                new_content = mod_pattern.sub(f"pub mod {module_name};", content)
                mod_file.write_text(new_content)
                self.logger.info(f"Módulo {module_name} cambiado a público en {mod_file}")
                return True
        
        return False
    
    async def _fix_missing_field(self, error_info: Dict) -> bool:
        """Soluciona campos faltantes en inicializadores de estructuras"""
        match = error_info["match"]
        field_name = match.group(1)
        struct_name = match.group(2)
        file_path = error_info["file"]
        
        if not file_path:
            return False
        
        self.logger.info(f"Reparando: Campo faltante '{field_name}' en estructura '{struct_name}'")
        
        # Buscar la definición de la estructura en los archivos del proyecto
        struct_def_file = None
        for rs_file in self.project_dir.rglob("**/*.rs"):
            content = rs_file.read_text()
            if re.search(fr"struct\s+{struct_name}\s*{{", content):
                struct_def_file = rs_file
                break
        
        if struct_def_file:
            # Si encontramos la definición, consultamos a Claude para añadir el campo correctamente
            prompt = f"""
Estoy intentando solucionar un error en un código Rust. La estructura '{struct_name}' tiene un campo faltante '{field_name}'.
Este es el error de compilación:
```
missing field `{field_name}` in initializer of `{struct_name}`
```

Por favor, proporciona únicamente el código para añadir este campo a la estructura (sin explicaciones):
1. La definición del campo a añadir.
2. Cómo inicializar este campo en los constructores de la estructura.
"""
            async with aiohttp.ClientSession() as session:
                field_solution = await self._consult_claude(session, prompt)
                
                # Implementar la solución
                # Esta es una solución simplificada, en un caso real
                # sería necesario un análisis sintáctico más sofisticado
                content = Path(file_path).read_text()
                pattern = fr"({struct_name}\s*{{\s*(?:[^}}]+))\s*}}"
                replacement = f"\\1, {field_name}: Default::default() }}"
                modified_content = re.sub(pattern, replacement, content)
                
                Path(file_path).write_text(modified_content)
                self.logger.info(f"Añadido campo faltante '{field_name}' a '{struct_name}' en {file_path}")
                return True
        
        return False
    
    async def _fix_undeclared_variable(self, error_info: Dict) -> bool:
        """Soluciona variables no declaradas"""
        match = error_info["match"]
        var_name = match.group(1)
        file_path = error_info["file"]
        line_num = error_info["line"]
        
        if not file_path or not line_num:
            return False
        
        self.logger.info(f"Reparando: Variable no declarada '{var_name}' en {file_path}:{line_num}")
        
        # Consultar a Claude para la mejor solución
        file_content = Path(file_path).read_text()
        file_lines = file_content.split('\n')
        
        # Extraer contexto cercano a la línea con error
        start_line = max(0, line_num - 10)
        end_line = min(len(file_lines), line_num + 10)
        context = '\n'.join(file_lines[start_line:end_line])
        
        prompt = f"""
Estoy intentando solucionar un error en un código Rust. La variable '{var_name}' no está declarada en el ámbito.
Este es el error de compilación:
```
cannot find value `{var_name}` in this scope
```

Contexto del código (alrededor de la línea {line_num}):
```rust
{context}
```

Por favor, proporciona únicamente la mejor solución para declarar o inicializar esta variable (sin explicaciones).
"""
        async with aiohttp.ClientSession() as session:
            solution = await self._consult_claude(session, prompt)
            
            # Implementar la solución
            # Simplificado: insertamos la declaración justo antes de la línea problemática
            fix_line = f"let {var_name} = Default::default(); // Autogenerado por reparador"
            file_lines.insert(line_num - 1, fix_line)
            modified_content = '\n'.join(file_lines)
            
            Path(file_path).write_text(modified_content)
            self.logger.info(f"Añadida declaración para variable '{var_name}' en {file_path}:{line_num}")
            return True

    async def _get_claude_assistance(self, full_error: str) -> bool:
        """Solicita asistencia a Claude para errores complejos"""
        self.logger.info("Solicitando asistencia a Claude para errores complejos")

        cargo_toml = (self.project_dir / "Cargo.toml").read_text()

        # Encontrar los archivos principales o problemáticos
        main_files = []
        src_dir = self.project_dir / "src"

        # Priorizar archivos principales
        for file_name in ["main.rs", "lib.rs", "index.rs"]:
            file_path = src_dir / file_name
            if file_path.exists():
                content = file_path.read_text()
                main_files.append({"path": str(file_path), "content": content})

        # Buscar archivos mencionados en el error
        error_file_pattern = re.compile(r'-->.*?([^:]+\.rs):\d+:\d+')
        for match in error_file_pattern.finditer(full_error):
            file_path = Path(match.group(1))
            if file_path.exists() and str(file_path) not in [f["path"] for f in main_files]:
                content = file_path.read_text()
                main_files.append({"path": str(file_path), "content": content})

        # Limitar a 3 archivos para no sobrecargar el contexto
        main_files = main_files[:3]

        prompt = f"""
Necesito ayuda para reparar un proyecto Rust que está fallando al compilar.

# Error de compilación:
```
{full_error}
```

# Cargo.toml:
```toml
{cargo_toml}
```

# Archivos principales:
{self._format_files_for_prompt(main_files)}

Por favor, analiza los errores y proporciona soluciones concretas. Para cada solución, indica:
1. El archivo que debe modificarse
2. El código exacto a cambiar
3. El código nuevo que debe reemplazarlo o agregarse

Evita explicaciones teóricas y céntrate en soluciones prácticas y específicas.
"""

        async with aiohttp.ClientSession() as session:
            solution = await self._consult_claude(session, prompt)

            # Guardar la respuesta en un archivo para referencia
            solutions_file = self.project_dir / "claude_solutions.md"
            solutions_file.write_text(solution)

            # Intentar aplicar las soluciones automáticamente
            await self._apply_claude_solutions(solution)

            self.logger.info(f"Soluciones de Claude guardadas en {solutions_file}")
            return True

    async def _apply_claude_solutions(self, solution_text: str) -> bool:
        """Intenta aplicar automáticamente las soluciones de Claude"""
        # Buscar bloques de código con la ruta del archivo
        file_pattern = re.compile(r'([^`\n]+\.rs)[^\n]*\n```(?:rust)?\n(.*?)\n```', re.DOTALL)

        fixes_applied = False

        for match in file_pattern.finditer(solution_text):
            file_path = match.group(1).strip()
            new_code = match.group(2)

            # Normalizar ruta del archivo
            if not file_path.startswith('/'):
                file_path = str(self.project_dir / file_path)

            file_path = Path(file_path)

            # Si el archivo no existe pero Claude sugiere crearlo
            if not file_path.exists() and "Crear nuevo archivo" in solution_text[:match.start()]:
                self.logger.info(f"Creando nuevo archivo: {file_path}")
                file_path.parent.mkdir(parents=True, exist_ok=True)
                file_path.write_text(new_code)
                fixes_applied = True
                continue

            # Si el archivo existe, verificar si Claude sugiere reemplazarlo completamente
            if file_path.exists():
                file_content = file_path.read_text()

                # Si Claude sugiere cambiar todo el archivo
                if "Reemplazar todo el archivo" in solution_text[:match.start()]:
                    self.logger.info(f"Reemplazando contenido completo de: {file_path}")
                    file_path.write_text(new_code)
                    fixes_applied = True
                    continue

                # Buscar cambios específicos
                # Esto es más complicado y requeriría un análisis más detallado
                # Esta es una versión simplificada
                changes = self._identify_changes(file_content, new_code)
                if changes:
                    for old, new in changes:
                        if old in file_content:
                            self.logger.info(f"Aplicando cambio específico en: {file_path}")
                            file_content = file_content.replace(old, new)
                            fixes_applied = True

                    file_path.write_text(file_content)

        return fixes_applied

    def _identify_changes(self, original: str, suggested: str) -> List[Tuple[str, str]]:
        """Identifica cambios específicos entre el código original y el sugerido"""
        changes = []

        # Método simple: dividir por líneas y encontrar bloques diferentes
        original_lines = original.split('\n')
        suggested_lines = suggested.split('\n')

        # Usar un algoritmo básico para encontrar bloques diferentes
        # Esto es una simplificación; un algoritmo real de diff sería más preciso
        i, j = 0, 0
        while i < len(original_lines) and j < len(suggested_lines):
            # Si las líneas coinciden, avanzar ambos índices
            if original_lines[i] == suggested_lines[j]:
                i += 1
                j += 1
                continue

            # Buscar el próximo punto de sincronización
            sync_i, sync_j = i, j
            while sync_i < len(original_lines) and sync_j < len(suggested_lines):
                # Buscar líneas que coincidan nuevamente
                for look_ahead in range(1, 5):  # Buscar hasta 5 líneas adelante
                    if sync_i + look_ahead < len(original_lines) and sync_j < len(suggested_lines) and \
                       original_lines[sync_i + look_ahead] == suggested_lines[sync_j]:
                        # Encontramos sincronización, este es un bloque a reemplazar
                        old_block = '\n'.join(original_lines[i:sync_i + look_ahead])
                        new_block = '\n'.join(suggested_lines[j:sync_j])
                        changes.append((old_block, new_block))
                        i = sync_i + look_ahead
                        j = sync_j
                        break
                    if sync_i < len(original_lines) and sync_j + look_ahead < len(suggested_lines) and \
                       original_lines[sync_i] == suggested_lines[sync_j + look_ahead]:
                        # Encontramos sincronización, este es un bloque a reemplazar
                        old_block = '\n'.join(original_lines[i:sync_i])
                        new_block = '\n'.join(suggested_lines[j:sync_j + look_ahead])
                        changes.append((old_block, new_block))
                        i = sync_i
                        j = sync_j + look_ahead
                        break

                sync_i += 1
                sync_j += 1

                # Si hemos buscado demasiado lejos sin éxito
                if sync_i >= i + 10 or sync_j >= j + 10:
                    break

            # Si no encontramos sincronización, avanzar manualmente
            i += 1
            j += 1

        return changes

    async def _generate_analysis_report(self, error_output: str):
        """Genera un informe de análisis del proyecto y recomendaciones"""
        project_structure = self._get_project_structure()
        cargo_toml = (self.project_dir / "Cargo.toml").read_text()

        prompt = f"""
Genera un análisis detallado para un proyecto Rust que no puedo compilar completamente.

# Error de compilación:
```
{error_output}
```

# Estructura del proyecto:
```
{project_structure}
```

# Cargo.toml:
```toml
{cargo_toml}
```

Por favor, proporciona:

1. Diagnóstico: ¿Cuáles son los problemas fundamentales del proyecto?
2. Recomendaciones: Pasos específicos para corregir la estructura del proyecto.
3. Ejemplo de código: Cualquier archivo clave que deba crear o modificar.
4. Plan de refactorización: Sugerencias para mejorar la organización del código.

Sé específico y práctico, centrándote en la arquitectura de Rust y las mejores prácticas.
"""

        async with aiohttp.ClientSession() as session:
            analysis = await self._consult_claude(session, prompt)

            # Guardar el análisis en un archivo
            analysis_file = self.project_dir / "project_analysis.md"
            analysis_file.write_text(analysis)

            self.logger.info(f"Análisis del proyecto guardado en {analysis_file}")

    def _get_project_structure(self) -> str:
        """Obtiene una representación de la estructura de directorios del proyecto"""
        result = []

        for root, dirs, files in os.walk(self.project_dir):
            level = root.replace(str(self.project_dir), '').count(os.sep)
            indent = ' ' * 4 * level
            result.append(f"{indent}{os.path.basename(root)}/")

            sub_indent = ' ' * 4 * (level + 1)
            for file in files:
                if file.endswith('.rs') or file == 'Cargo.toml':
                    result.append(f"{sub_indent}{file}")

        return '\n'.join(result)

    def _format_files_for_prompt(self, files: List[Dict]) -> str:
        """Formatea la lista de archivos para incluir en el prompt"""
        result = []

        for file in files:
            result.append(f"## {file['path']}:\n```rust\n{file['content']}\n```\n")

        return '\n'.join(result)

    async def _consult_claude(self, session: aiohttp.ClientSession, prompt: str) -> str:
        """Consulta a Claude para obtener soluciones"""
        claude_url = "https://api.anthropic.com/v1/messages"

        headers = {
            "x-api-key": self.api_key,
            "anthropic-version": "2023-06-01",
            "content-type": "application/json"
        }

        system_message = "Eres un experto en Rust con experiencia en solución de problemas de compilación y arquitectura de proyectos. Tu tarea es proporcionar soluciones prácticas y específicas para corregir errores de compilación y estructurar correctamente proyectos Rust."

        data = {
            "model": "claude-3-7-sonnet-20250219",
            "max_tokens": 4000,
            "system": system_message,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }

        # Estrategia de reintento con backoff exponencial
        max_retries = 5
        base_delay = 2

        for attempt in range(max_retries):
            try:
                async with self.semaphore:  # Controlar concurrencia
                    async with session.post(claude_url, headers=headers, json=data) as response:
                        response_text = await response.text()

                        if response.status == 200:
                            response_data = json.loads(response_text)
                            return response_data["content"][0]["text"]

                        elif response.status == 429:  # Límite de tasa
                            self.logger.warning(f"Límite de tasa de API (intento {attempt+1}/{max_retries})")

                            if attempt >= max_retries - 1:
                                self.logger.error("Se alcanzó el número máximo de reintentos")
                                return "Error: No se pudo completar la solicitud debido a límites de tasa"

                            # Backoff exponencial
                            delay = (base_delay ** (attempt + 1)) + random.uniform(0, 5)
                            self.logger.info(f"Esperando {delay:.2f} segundos antes de reintentar...")
                            await asyncio.sleep(delay)

                        else:
                            self.logger.error(f"Error en API: {response.status} - {response_text}")

                            if attempt >= max_retries - 1:
                                self.logger.error("Se alcanzó el número máximo de reintentos")
                                return f"Error: La API respondió con status {response.status}"

                            # Esperar antes de reintentar
                            delay = base_delay ** attempt
                            await asyncio.sleep(delay)

            except aiohttp.ClientError as e:
                self.logger.error(f"Error de conexión: {str(e)}")

                if attempt >= max_retries - 1:
                    self.logger.error("Se alcanzó el número máximo de reintentos")
                    return "Error: Problemas de conexión con la API"

                await asyncio.sleep(base_delay ** attempt)

        return "Error: No se pudo obtener respuesta después de múltiples intentos"


def create_main_file(project_dir: Path) -> bool:
    """Utilitario para crear un archivo main.rs básico"""
    src_dir = project_dir / "src"
    main_file = src_dir / "main.rs"

    if main_file.exists():
        return False

    # Crear contenido básico para main.rs
    content = """fn main() {
    println!("Hello, Rust project!");

    // Aquí inicia la aplicación
    // Código autogenerado por el RustRepairAgent
}
"""
    main_file.write_text(content)
    return True


def modify_cargo_toml(project_dir: Path) -> bool:
    """Utilitario para modificar Cargo.toml si es necesario"""
    cargo_path = project_dir / "Cargo.toml"

    if not cargo_path.exists():
        return False

    content = cargo_path.read_text()

    # Verificar si ya tiene [[bin]] o [lib]
    if "[[bin]]" in content or "[lib]" in content:
        return False

    # Verificar si tiene archivos principales
    has_main = (project_dir / "src" / "main.rs").exists()
    has_lib = (project_dir / "src" / "lib.rs").exists()

    # Si no tiene ninguno, agregar configuración de binario
    if not has_main and not has_lib:
        if create_main_file(project_dir):
            # Agregar sección de binarios si creamos main.rs
            content += "\n[[bin]]\nname = \"main\"\npath = \"src/main.rs\"\n"
            cargo_path.write_text(content)
            return True

    return False


async def main():
    # Configuración
    api_key = os.getenv("ANTHROPIC_API_KEY")
    project_dir = os.getenv("PROJECT_DIR", ".")
    auto_fix = os.getenv("AUTO_FIX", "True").lower() != "false"
    
    if not api_key:
        print("Advertencia: La variable de entorno ANTHROPIC_API_KEY no está configurada")
        print("Se ejecutará sin asistencia avanzada de Claude")
    
    # Crear y ejecutar el agente de reparación mejorado
    repair_agent = EnhancedRustRepairAgent(api_key, project_dir, auto_fix=auto_fix)
    await repair_agent.repair_codebase()


def crear_script_shell():
    """Crea un script de shell para ejecutar el reparador fácilmente"""
    script_content = """#!/bin/bash
# Script para ejecutar el reparador de código Rust

# Colores para la salida
GREEN='\\033[0;32m'
YELLOW='\\033[1;33m'
RED='\\033[0;31m'
NC='\\033[0m' # No Color

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
"""
    
    script_name = "repair-rust.sh"
    with open(script_name, "w") as f:
        f.write(script_content)
    os.chmod(script_name, 0o755)  # Hacer ejecutable
    print(f"Script de shell creado: {script_name}")


if __name__ == "__main__":
    import asyncio
    
    # Ejecutar el programa principal
    asyncio.run(main())
    
    # Crear script de shell para facilitar su uso
    crear_script_shell()
