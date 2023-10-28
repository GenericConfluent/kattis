RUST := $(wildcard *.rs)
HASKELL := $(wildcard *.hs)
TARGET := target

.PHONY: all
all: clean rs hs

.PHONY: hs
hs: $(HASKELL:.hs=)

.PHONY: rs
rs: $(RUST:.rs=)

clean:
	rm -rf target
	mkdir target

%: %.rs
	rustc $< -o $(TARGET)/$@

%: %.hs
	ghc $< -o $(TARGET)/$@

