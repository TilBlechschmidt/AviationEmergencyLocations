# ----- WASM builder
FROM rust:1.54 AS wasm

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the wasm package
COPY web-app/elsa /app
RUN cd /app && wasm-pack build -t web -d target/pkg

# ----- Web builder
FROM node:14-alpine3.14 AS web

WORKDIR /app
COPY web-app .
RUN npm install
COPY --from=wasm /app/target/pkg /app/elsa/target/pkg
RUN npm run build

# ----- Delivery
FROM caddy:2-alpine
COPY --from=web /app/build /usr/share/caddy
