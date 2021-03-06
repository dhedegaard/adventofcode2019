FROM rust AS wasm-build
WORKDIR /app

# Install wasm-pack
RUN sh -c "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh;"

# Add the rust code.
COPY ./aoc2019 ./

# Run the tests.
RUN cargo test --release

# Build it.
RUN wasm-pack build --out-dir /output --target=web

FROM node AS gatsby-build
WORKDIR /app

# Add the source and the wasm build.
COPY . ./
COPY --from=wasm-build /output ./static/aoc2019

# Build output to "/app/public"
RUN yarn && yarn start

# Serve the thing with nginx.
FROM nginx
COPY --from=gatsby-build /app/public /usr/share/nginx/html
