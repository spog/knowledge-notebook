Set the development environment:
-------------------------------
$ cd knowledge-notebook/
knowledge-notebook$

knowledge-notebook$ nix develop
🚀 Dev Environment Ready!
rustc 1.90.0 (1159e78c4 2025-09-14)
deno 2.5.1 (stable, release, x86_64-unknown-linux-gnu)
v8 14.0.365.4-rusty
typescript 5.9.2
knowledge-notebook$ 

Build backend:
-------------
(...inside nix develop...)
knowledge-notebook$ cd backend/
knowledge-notebook/backend$ 

knowledge-notebook/backend$ cargo build
...

Start the DB instance (postgresql in a docker container):
--------------------------------------------------------
(...outside nix develop...)
$ cd knowledge-notebook/
knowledge-notebook$

knowledge-notebook$ docker compose up -d
WARN[0000] /home/samo/GITHUB/knowledge-notebook/docker-compose.yml: the attribute `version` is obsolete, it will be ignored, please remove it to avoid potential confusion 
[+] Running 1/1
 ✔ Container notebook_db  Started                                                                                                                                                                                                3.8s 
knowledge-notebook$ 

knowledge-notebook$ docker ps
CONTAINER ID   IMAGE         COMMAND                  CREATED       STATUS         PORTS                                       NAMES
cb21d814061e   postgres:15   "docker-entrypoint.s…"   13 days ago   Up 4 minutes   0.0.0.0:5432->5432/tcp, :::5432->5432/tcp   notebook_db
knowledge-notebook$ 

Stop the container DB instance:
------------------------------
(...outside nix develop...)
knowledge-notebook$ docker compose down
WARN[0000] /home/samo/GITHUB/knowledge-notebook/docker-compose.yml: the attribute `version` is obsolete, it will be ignored, please remove it to avoid potential confusion 
[+] Running 2/2
 ✔ Container notebook_db               Removed                                                                                                                                                                                   2.3s 
 ✔ Network knowledge-notebook_default  Removed                                                                                                                                                                                   0.9s 
knowledge-notebook$ 

Rebuild the container DB instance:
---------------------------------
(...outside nix develop...)
knowledge-notebook$ docker compose up -d --build
[+] Running 2/2
 ✔ Network knowledge-notebook_notebook-net  Created                                                                                                                                                                              0.7s 
 ✔ Container notebook_db                    Started                                                                                                                                                                              3.5s 
knowledge-notebook$ 

Run backend:
-----------
(...inside nix develop...)
knowledge-notebook$ cd backend/
knowledge-notebook/backend$ 

knowledge-notebook/backend$ cargo run
...
     Running `target/debug/backend`
2025-10-11T10:30:15.318121Z  INFO backend: 🚀 Backend running at http://127.0.0.1:3000

