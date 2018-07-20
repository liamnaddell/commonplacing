all:
	./helmcmd
	kubectl create -f .

dockerfile:
	docker build -t qlek/commonplacing:1.4 .

push:
	docker push qlek/commonplacing:1.4

stop:
	helm del --purge mineql
	kubectl delete -f .
