EXT := rs
EXE_FILES := $(shell ls *.$(EXT) | \
               sed -e 's/\.$(EXT)/\.exe/' \
             )


all: $(EXE_FILES)

%.exe: %.$(EXT)
	rustc $<

