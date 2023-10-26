all: clean froggerhard froggerhard_old

clean:
	rm -rf target
	mkdir target

froggerhard:
	rustc froggerhard.rs -o target/froggerhard

froggerhard_old:
	rustc froggerhard_old.rs -o target/froggerhard_old
