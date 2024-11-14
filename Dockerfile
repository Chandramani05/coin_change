FROM rust:1.82 AS build


WORKDIR /app
COPY . .
RUN cargo build --release

# Distroless runtime stage
FROM gcr.io/distroless/static-debian12
COPY --from=build /app/target/release/coin_change /app/

# Use non-root user
USER nonroot:nonroot

# Set up app directory
ENV APP_HOME=/app
WORKDIR $APP_HOME

# Expose port
EXPOSE 3000

# Run app
ENTRYPOINT ["/app/coin_change"]
