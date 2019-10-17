FROM rustlang/rust:nightly-stretch-slim
COPY . .
RUN apt-get update -y && apt-get install -y libpq-dev
# Non production image
CMD ["cargo", "run"]