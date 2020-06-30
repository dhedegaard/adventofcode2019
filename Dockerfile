FROM rust AS wasm-build
WORKDIR /app

# Install wasm-pack
RUN sh -c "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh;"

# Add the rust code.
COPY ./aoc2019 ./

# Build it.
RUN  wasm-pack build --out-dir /output --target=web

FROM node
WORKDIR /app

# Add the source and the wasm build.
COPY . ./
COPY --from=wasm-build /output ./static/aoc2019

# Build output to "/app/public"
RUN yarn && yarn start
