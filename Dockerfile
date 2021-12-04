# ----- WASM builder
FROM rust:1.54 AS wasm

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the wasm package
COPY elsa /app
RUN cd /app && wasm-pack build -t web -d target/pkg

# ----- Web builder
FROM node:14-alpine3.14 AS web

WORKDIR /app
COPY . .
RUN npm install
COPY --from=wasm /app/target/pkg /app/elsa/target/pkg
RUN npm run build

# ----- Delivery
FROM node:17-alpine3.12

WORKDIR /app
COPY --from=web /app/build .
COPY --from=web /app/package.json .

EXPOSE 3000
CMD ["node", "index.js"]