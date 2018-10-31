FROM mhart/alpine-node:10 as base
WORKDIR /usr/src
COPY package.json yarn.lock ./
RUN yarn --production

FROM mhart/alpine-node:base-10
WORKDIR /usr/src
COPY --from=base /usr/src .
COPY . .
CMD node index.js
