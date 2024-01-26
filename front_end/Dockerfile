# syntax=docker/dockerfile:1

FROM node:21.4.0-alpine3.19 AS builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
ENV PUBLIC_INTERNAL_SERVER_URL=http://proxy:8080/backend
ENV PUBLIC_EXTERNAL_SERVER_URL=https://sie.blazanome.ynh.fr/backend
RUN npm run build
RUN npm ci --omit=dev

FROM node:21.4.0-alpine3.19
USER node:node
WORKDIR /app
ENV PUBLIC_INTERNAL_SERVER_URL=http://proxy:8080/backend
ENV PUBLIC_EXTERNAL_SERVER_URL=https://sie.blazanome.ynh.fr/backend
COPY --from=builder --chown=node:node /app/build ./build
COPY --from=builder --chown=node:node /app/node_modules ./node_modules
COPY --chown=node:node package.json .
CMD ["node","build"]
