all: compile  deploy

compile:
	cargo run ./protos/game.proto 

deploy:
	cp *.rs ../smallgameserver/src
