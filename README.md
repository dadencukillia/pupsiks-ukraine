# Currently in pre-release state
## To-Do:
- [x] Make API backend endpoints
- - [x] Get certificate endpoint
- - [x] Stats endpoint
- - [x] Email confirmation code sending endpoint (+redis)
- - [x] Create certificate endpoint
- - [x] Remove certificate endpoint
- - [x] Forgot certificate endpoint
- [x] Connect frontend and backend
- [x] Make skeleton
- [x] Separate components on svelte files
- [x] Turn backend and frontend into Docker services
- - [x] Backend
- - [x] Frontend
- [x] Make .env disappear (.gitignore magic)
- [x] Refactor the code
- - [x] Backend
- - [x] Frontend
- [ ] Add comments in the code
- - [ ] Backend
- - [x] Frontend
- [ ] Add developement mode
- [ ] Add backend tests
- [ ] CI/CD
- [ ] Make a great README.md
- [ ] ~Remove all commits (hide my shame)~
- [ ] Publish the project

# Association pupsiks of Ukraine
The revolutional platform for the cutest people of Ukraine made with love.

## Technologies
- Frontend:
- - SvelteKit
- - Tailwind
- Backend:
- - Actix
- - SeaORM
- DB:
- - PostgreSQL
- Also:
- - Redis
- - SMTP with Golang

## Run project
Type the command in the root directory:
```bash
sudo docker compose up --build
```

Go to [http://localhost](http://localhost)
