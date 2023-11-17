RUST := $(wildcard *.rs)
HASKELL := $(wildcard *.hs)
TARGET := target

.PHONY: all
all: clean rs hs

.PHONY: hs
hs: $(HASKELL:.hs=) posths

.PHONY: rs
rs: $(RUST:.rs=)

.PHONY: posths
posths:
	@rm *.o
	@rm *.hi

.PHONY: clean
clean:
	rm -rf target
	@mkdir target

%: %.rs
	rustc $< -o $(TARGET)/$@

%: %.hs
	ghc $< -o $(TARGET)/$@

