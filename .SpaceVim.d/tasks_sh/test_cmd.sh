prob=$(perl -sE '@s=split(/\//, $fd_name);say $s[0]' -- -fd_name=$1)
prop_p=$(perl -sE '@s=split(/\//, $fd_name);say $s[1]' -- -fd_name=$1)
container_hash=$(docker ps | perl -anle 'if(/atcoder_dev_env_image/){print($F[0])}')
docker exec $container_hash /bin/bash -c "cd /home/atcoder/$prob; make test q=$prop_p"
