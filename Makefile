PHONY: create

setup:
	sudo chown -R vscode:vscode /home/vscode/.config
	acc config default-template cpp
	acc login
