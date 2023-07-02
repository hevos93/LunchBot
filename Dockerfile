FROM alpine
WORKDIR /app
COPY target/release/lunchbot lunchbot
# This is required for Alpine to run the binary
RUN apk add --no-cache gcompat libgcc
CMD [ "./lunchbot" ]