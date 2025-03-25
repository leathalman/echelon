# build stage: Node.js 22 Alpine for minimal size
FROM node:22-alpine AS builder
WORKDIR /app

# copy package files and install dependencies
COPY package*.json ./
RUN npm ci

# copy source files and build app
COPY . .
RUN npm run build
RUN npm prune --production

# runtime stage: fresh Node.js Alpine image
FROM node:22-alpine
WORKDIR /app

COPY .env.production .env.production

# copy only the necessary files from builder
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY package.json .

# configure container
EXPOSE 3000
ENV NODE_ENV=production
CMD [ "node", "build" ]