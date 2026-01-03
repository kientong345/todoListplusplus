# Build stage
FROM node:20-slim AS builder

WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Copy source code
COPY . .

# Build args for environment variables
ARG VITE_API_URL='/api'
ENV VITE_API_URL=$VITE_API_URL
ARG VITE_GOOGLE_CLIENT_ID='420955407480-ojgfvv6qbif268ee7r9rjsr0l1hbnke2.apps.googleusercontent.com'
ENV VITE_GOOGLE_CLIENT_ID=$VITE_GOOGLE_CLIENT_ID

# Build the application
RUN npm run build

# Runtime stage
FROM nginx:alpine

# Copy the built static files to Nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy a custom nginx configuration to handle SPA routing if necessary
COPY ./nginx/reverse_proxy.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
