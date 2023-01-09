build problem:
  docker build -t {{problem}} --build-arg APP_NAME={{problem}} .

run problem: (build problem)
  docker run -p 7070:7070 --rm -it {{problem}}

deploy problem: (build problem)
  flyctl deploy --local-only --image {{problem}}
