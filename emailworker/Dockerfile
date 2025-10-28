FROM golang:1.25.3-alpine3.22 AS build-stage

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY *.go ./

RUN GOOS=linux go build -o /emailworker


FROM alpine:3.22 AS build-release-stage

WORKDIR /app

COPY --from=build-stage /emailworker /app/emailworker
COPY . .

CMD ["./emailworker"]
