PHONY: create

setup:
	cp -r ./templates/python $(HOME)/.config/atcoder-cli-nodejs/
	acc config default-template python
	acc login
