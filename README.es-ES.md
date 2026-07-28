# ✰ArtSCII✰

Un espacio de trabajo con un crate de base compartido, un crate reutilizable de conversión de imágenes, un binario de CLI y un andamiaje para video.

## Características

- **Múltiples algoritmos de tramado (dithering)**: Floyd-Steinberg, Atkinson y Riemersma
- **Soporte de color**: Colores ANSI para terminal y salida de texto, RGB para salida HTML
- **Múltiples formatos de salida**: Terminal, texto plano, texto coloreado, HTML estilizado, GIF y MP4
- **Conversión de video a animación ASCII**: Convierte video a ASCII animado en terminal, GIF o MP4
- **Ajustes de imagen**: Controles de resolución, contraste y brillo
- **Amplio soporte de formatos**: PNG, JPG, GIF, BMP, WebP y más
- **Basado en navegador**: Convierte imágenes a arte ASCII totalmente del lado del cliente a través de WebAssembly

## Espacio de Trabajo

El proyecto está organizado como un workspace de Cargo con los siguientes crates:

| Crate                                   | Descripción                                     | Tipo          |
| --------------------------------------- | ----------------------------------------------- | ------------- |
| [`artscii-core`](crates/artscii-core)   | Configuración compartida, errores y estrategias de tramado | Librería      |
| [`artscii-img`](crates/artscii-img)     | Carga de imágenes y conversión a ASCII           | Librería      |
| [`artscii-video`](crates/artscii-video) | Decodificación de video y codificación ASCII     | Librería      |
| [`artscii-web`](crates/artscii-web)     | Convertidor WASM basado en navegador            | Binario (WASM) |
| [`artscii-cli`](crates/artscii-cli)     | Interfaz de línea de comandos                   | Binario       |

Los tres crates de librería (`core`, `img`, `video`) están diseñados para ser utilizables desde otros proyectos de Rust. Consulta el README de cada crate para más detalles.

## Uso del crate

Usa `artscii-img` desde otro proyecto de Rust como una dependencia de ruta local:

```bash
cargo add --git https://github.com/4ster-light/artscii artscii-img
# Si deseas bloquearlo a una rama, etiqueta o commit:
cargo add --git https://github.com/4ster-light/artscii --branch main artscii-img
cargo add --git https://github.com/4ster-light/artscii --tag v1.1.0 artscii-img
cargo add --git https://github.com/4ster-light/artscii --rev <commit-sha> artscii-img
```

O, en `Cargo.toml`:

```toml
[dependencies]
artscii-img = { git = "https://github.com/4ster-light/artscii", branch = "main" }
```

Lo mismo aplica para `artscii-core` si deseas usar los tipos y la configuración compartidos, o para `artscii-video`.

El crate CLI construye el binario `artscii`.

## Instalación

### Binarios Precompilados

Descarga los binarios precompilados para tu plataforma desde el
[último lanzamiento](https://github.com/4ster-light/artscii/releases/latest):

- **Linux x86_64**: `artscii-linux-x86_64`
- **Linux ARM64**: `artscii-linux-aarch64`
- **macOS x86_64**: `artscii-macos-x86_64`
- **macOS ARM64**: `artscii-macos-aarch64`
- **Windows x86_64**: `artscii-windows-x86_64.exe`

Después de descargar, extrae y ejecuta (en Linux / Unix):

```bash
# Linux (tar.gz con librerías incluidas)
tar xzf artscii-linux-x86_64.tar.gz
./artscii --help

# macOS / Windows
chmod +x artscii-macos-x86_64
./artscii-macos-x86_64
```

> [!NOTE]
> Los archivos tar de Linux incluyen las librerías ffmpeg/alsa necesarias; no se requieren paquetes adicionales en tiempo de ejecución. macOS y Windows incluyen el soporte de video de forma nativa.

### Usando Nix

Instala en tu perfil:

```bash
nix profile add github:4ster-light/artscii
```

O añádelo a la configuración de tu sistema si usas NixOS.

### Cargo

Construye e instala la CLI desde el código fuente:

```bash
cargo install --git https://github.com/4ster-light/artscii artscii-cli
```

## Uso

### Opciones

| Bandera | Largo               | Descripción                         | Predeterminado |
| -------- | ------------------- | ----------------------------------- | -------------- |
| `-o`     | `--output`          | Ruta del archivo de salida          | stdout         |
| `-f`     | `--format`          | Formato de salida (terminal/text/html) | auto           |
| `-r`     | `--resolution`     | Factor de escala (0.01-1.0)         | 0.3            |
|          | `--contrast`       | Contraste (0.1-3.0)                 | 1.0            |
| `-b`     | `--brightness`     | Brillo (0.1-3.0)                    | 1.0            |
| `-i`     | `--invert`          | Invertir mapeo de caracteres         | false          |
| `-c`     | `--color`           | Habilitar salida coloreada           | false          |
| `-d`     | `--dithering`       | Algoritmo de tramado                | none           |
| `-q`     | `--quiet`           | Suprimir mensajes de información     | false          |
|          | `--video`           | Habilitar modo video               | false          |
|          | `--video-format`    | Salida de video (terminal/gif/mp4)  | terminal      |
|          | `--preserve-audio`  | Mantener audio original en archivos de salida | false |

### Ejemplos de Comandos

```bash
# Uso básico - mostrar en terminal
artscii image.jpg

# Con colores
artscii image.jpg -c

# Guardar como texto plano
artscii image.jpg -o output.txt

# Guardar texto con colores ANSI
artscii image.jpg -c -o output.txt

# Guardar como HTML estilizado con colores
artscii image.jpg -o output.html -c

# Mayor resolución con tramado Atkinson
artscii image.jpg -r 0.5 -d atkinson

# Ajustar contraste y brillo
artscii image.jpg --contrast 1.5 -b 0.8

# Invertir caracteres (para fondos claros)
artscii image.jpg -i

# Modo silencioso (solo muestra el arte)
artscii image.jpg -q

# Reproducir video como animación ASCII en terminal
artscii video.mp4 --video

# Reproducir video en terminal con colores
artscii video.mp4 --video -c

# Exportar video como GIF animado (coloreado, mayor resolución)
artscii video.mp4 --video --video-format gif -c -r 0.5 -o output.gif

# Exportar video como MP4 (coloreado, con audio original)
artscii video.mp4 --video --video-format mp4 -c --preserve-audio -o output.mp4
```

> Y lo que considero el mejor resultado para la imagen de ejemplo en este repositorio:
>
> ```bash
> artscii image.jpg -c -b 1.5 -r 0.26 -d atkinson -o img.html
> ```

### Algoritmos de Tramado (Dithering)

- **none**: Sin tramado (predeterminado)
- **floyd-steinberg**: Tramado clásico de difusión de error
- **atkinson**: Tramado más nítido (preserva más el contraste)
- **riemersma**: Tramado de curva de llenado de espacio

## Construcción desde el Código Fuente

Construir desde el código fuente requiere librerías del sistema para el soporte de video y audio.

### Dependencias

| Plataforma | Dependencias de construcción                                                                                        |
| ----------- | ------------------------------------------------------------------------------------------------------------------- |
| Nix         | `nix develop` (todas las deps incluidas)                                                                             |
| Fedora      | `ffmpeg-devel alsa-lib-devel clang-devel`                                                                             |
| Debian      | `libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libswscale-dev libasound2-dev libclang-dev`              |
| Arch        | `ffmpeg alsa-lib clang`                                                                                             |
| macOS       | `brew install ffmpeg`                                                                                                |

```bash
# Usando Nix (recomendado)
nix build

# O instala las deps manualmente y luego:
cargo build --release
```

## Licencia

MIT
