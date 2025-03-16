FROM ruby:3.2.6

WORKDIR /app

RUN apt-get update -qq && apt-get install -y \
  curl build-essential libclang-dev \
  unixodbc-dev odbc-postgresql
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add aarch64-unknown-linux-gnu 

# Add the include path for the clang headers
ENV BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include/aarch64-linux-gnu"

# Disable the crt-static feature to avoid linking to the static C runtime
# to avoid conflicts with the dynamic C runtime
ENV RUSTFLAGS="-C target-feature=-crt-static"

CMD ["bash", "-c", "while :; do sleep 1; done"]
