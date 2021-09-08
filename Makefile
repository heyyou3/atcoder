PHONY: create

setup:
	cp -r ./templates/go $(HOME)/.config/atcoder-cli-nodejs/
	acc config default-template go
	acc login
