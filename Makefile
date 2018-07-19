all:
	./helmcmd
	kubectl create -f .

stop:
	helm del --purge mineql
	kubectl delete -f .
