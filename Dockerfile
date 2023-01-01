# jdk eclipse temurin 19.0.1
# sbt 1.8.0
# scala 3.2.1
FROM sbtscala/scala-sbt:eclipse-temurin-19.0.1_10_1.8.0_3.2.1 AS builder

COPY . /app

WORKDIR /app

RUN sbt assembly

FROM eclipse-temurin:19.0.1_10-jre AS runtime

COPY --from=builder /app/target/scala-3.2.1/go2.jar .

CMD java -jar go2.jar