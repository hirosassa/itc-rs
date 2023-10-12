bash << EOS
#!/bin/bash
echo "restore start"
pg_restore -c -U postgres -d dvdrental /var/lib/postgresql/data/sample/dvdrental.tar &>/dev/null
echo "done"
EOS
