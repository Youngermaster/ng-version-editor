all: vet fmt build

vet:
	go vet main.go

fmt:
	gofmt main.go

build:
	go build -o build/ng-version-eraser main.go

clean:
	rm -rf build/