# IoT Anlytics (W.I.P.)

Este proyecto está destinado a desplegar una interface web, en la cual se visualizarán gráficos y métricas de datos obtenidos a partir de
distintos sensores (temperatura, humedad, distancia, presión, luminosidad, etc).

Está desarrollado en Rust para el backend y en Angular para el frontend. Tiene varias características, ya implementadas, para hacer
más efeciente el desarrollo y ejecución:

1. Este proyecto intenta imitar una estructura parecedida a Springboot en Rust. Para emular el *["live-reload"](https://docs.spring.io/spring-boot/docs/current/reference/html/using.html#using.running-your-application.hot-swapping)* de springboot, instalar [cargo-watch](https://actix.rs/docs/autoreload/) y luego ejecutar el script hotreload.sh.

2. La parte Angular tiene una configuración especial en el package.json (build script), que permite comprimir con el algorimo [Brotli](https://github.com/google/brotli) los artefactos (bundle) generados.

3. La implementación en Rust hace uso de [actix-web](https://actix.rs/) para la implementación de una API Rest. Además se utiliza [actix_files::Files](https://docs.rs/actix-files/latest/actix_files/) para el despliegue de contenido estático (el bundle de Angular, básicamente).

4. Hay un midleware para actix-web que permite agregar el header *["Content-Encoding: br"](https://developer.mozilla.org/es/docs/Web/HTTP/Headers/Content-Encoding)* a algunos archivos servidos estáticamente. Favor de revisar [esta pieza de código](src/back/http-utils/br_encoding.rs) para más detalles.
