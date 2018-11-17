# visumusic

detect notes and instruments in music

## Setup

build (in root folder)
> wasm-pack build

If npm has no access to your binary folders (so you get an EACCES error), execute the next command:
```
mkdir ~/.npm-global; npm config set prefix '~/.npm-global'
```

install dependencies (in www folder)
> npm install

link to use as local npm package

(in pkg folder)
> npm link

(in www folder)
> npm link visumusic

serve locally (in www folder)
> npm run start

look at your cool stuff (in browser)
> <http://localhost:8080/>

## Develop

always keep serving (in www folder)
> npm run start

to integrate changes just build again (in root folder)
> wasm-pack build
