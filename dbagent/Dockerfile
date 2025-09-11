FROM enspirit/dbagent:3.8.3

USER root

RUN bundle add pg && \
    mkdir -p /home/app/backups && \
    apt-get update && \
    apt-get install --yes --no-install-recommends postgresql-client && \
    rm -rf /var/lib/apt/lists/*

USER app

COPY migrations/   /home/app/migrations/
COPY data/         /home/app/data/
COPY backups/      /home/app/backups/
COPY tasks/        /home/app/tasks/

CMD bundle exec rackup -p 9292 -o 0.0.0.0
