#!/usr/bin/env bash
#
# Launch one or more OpenSearch nodes via the Docker image,
# to form a cluster suitable for running the REST API tests.
#
# Version 1.1.0
# - Initial version of the run-opensearch.sh script
# - Deleting the volume should not dependent on the container still running
# - Fixed `ES_JAVA_OPTS` config

script_path=$(dirname $(realpath -s $0))
source $script_path/functions/imports.sh
set -euo pipefail

echo -e "\033[34;1mINFO:\033[0m Take down node if called twice with the same arguments (DETACH=true) or on seperate terminals \033[0m"
cleanup_node $opensearch_node_name

master_node_name=${opensearch_node_name}
cluster_name=search-rest-test

declare -a volumes
environment=($(cat <<-END
  --env node.name=$opensearch_node_name
  --env cluster.name=$cluster_name
  --env cluster.initial_master_nodes=$master_node_name
  --env discovery.seed_hosts=$master_node_name
  --env cluster.routing.allocation.disk.threshold_enabled=false
  --env bootstrap.memory_lock=true
  --env node.attr.testattr=test
  --env path.repo=/tmp
  --env repositories.url.allowed_urls=http://snapshot.test*
  --env action.destructive_requires_name=false
END
))

NUMBER_OF_NODES=${NUMBER_OF_NODES-1}
http_port=9200
for (( i=0; i<$NUMBER_OF_NODES; i++, http_port++ )); do
  node_name=${opensearch_node_name}$i
  node_url=${external_opensearch_url/9200/${http_port}}
  if [[ "$i" == "0" ]]; then node_name=$opensearch_node_name; fi
  environment+=($(cat <<-END
    --env node.name=$node_name
END
))
  echo "$i: $http_port $node_url "
  volume_name=${node_name}-rest-test-data
  volumes+=($(cat <<-END
    --volume $volume_name:/usr/share/opensearch/data${i}
END
))

  # make sure we detach for all but the last node if DETACH=false (default) so all nodes are started
  local_detach="true"
  if [[ "$i" == "$((NUMBER_OF_NODES-1))" ]]; then local_detach=$DETACH; fi

  echo -e "\033[34;1mINFO: building $CLUSTER container\033[0m"
  echo 'cluster is' $CLUSTER
  docker build \
    --file=.ci/$CLUSTER/Dockerfile \
    --build-arg SECURE_INTEGRATION=$SECURE_INTEGRATION \
    --build-arg STACK_VERSION=$STACK_VERSION \
    --tag=$CLUSTER-secure-$SECURE_INTEGRATION \
    .

  echo -e "\033[34;1mINFO:\033[0m Starting container $node_name \033[0m"
  set -x
  if [[ "$SECURE_INTEGRATION" == "true" ]]; then
    healthcmd="curl -vvv -s --insecure -u admin:admin --fail https://localhost:9200/_cluster/health || exit 1"
  else
    healthcmd="curl -vvv -s --fail http://localhost:9200/_cluster/health || exit 1"
  fi

  docker run \
    --name "$node_name" \
    --network "$network_name" \
    --env "ES_JAVA_OPTS=-Xms1g -Xmx1g" \
    "${environment[@]}" \
    "${volumes[@]}" \
    --publish "$http_port":9200 \
    --ulimit nofile=65536:65536 \
    --ulimit memlock=-1:-1 \
    --detach="$local_detach" \
    --health-cmd="$(echo $healthcmd)" \
    --health-interval=2s \
    --health-retries=20 \
    --health-timeout=2s \
    --rm \
    -d \
    $CLUSTER-secure-$SECURE_INTEGRATION;

  set +x
  if wait_for_container "$opensearch_node_name" "$network_name"; then
    echo -e "\033[32;1mSUCCESS:\033[0m Running on: $node_url\033[0m"
  fi

done

