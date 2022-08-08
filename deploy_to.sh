#! /bin/bash

#! /bin/bash

# this script runs all deploy.sh in all needed services. If you add a service, you need to add it here.
# at last, it copies the file timeover_run.sh to the $HOME/timeover folder 

# global vars
file_name=`basename "$0"`
# default values
verbose=""
dry_run=""
current_directory=`pwd`


#working parameter
RED='\033[0;31m'
ORANGE='\033[0;33m'
NC='\033[0m' # No Color


usage(){
  echo "Usage: $file_name  [-h] [-s service] [-v]" 
  echo "    $file_name -h                      Display this help message."
  echo "    $file_name -l <location>           Server and location for rsync"
  echo "    $file_name -v <boolean>            Verbose to log actions."
  echo "    $file_name -d <boolean>            Dry-run and verbose to log actions."
  log "Exit on help request."
  exit 1
}

# logging 
log() {
  if [[ $1 == '-e' ]]; then
    echo $1 "${RED}[ERROR]${NC} $2"
    exit 0;
  fi
  if [[ $1 == '-w' ]]; then
    echo $1 "${ORANGE}[WARNING]${NC} $2"
  fi
  if [[ $verbose == "true" ]]; then 
    echo "[DEBUG] $1"
  fi
}

parameter_check () {
  optstring=":l:hv:d:"
  while getopts ${optstring} opt; do
    case ${opt} in
      v )
        verbose=$OPTARG
        log "Set debug to ${OPTARG}"
        ;;
      d )
        dry_run=$OPTARG
        log "Set dry-run to ${OPTARG}"
        ;;        
      l )
        log "Destination Location is ${OPTARG}"
        destination=$OPTARG
      ;;
      h )
        log "Help is requested."
        usage
        ;;      
      \? )
        log "-e" "Invalid Option: -$OPTARG" 1>&2
        log "Use Option -h for help."
        usage
      ;;
    esac
  done
}


copy_file () {
   source="${1}"
   destination="${2}"

   log "Run:  "
   if [[ $dry_run == "true" ]];  
    then
      log "     rsync --dry-run  -av --progress --exclude='.*' ${service} ${destination}"  
      rsync --dry-run -v -av --progress --exclude='.*' ${service} ${destination}
    else 
      log "     rsync -av --progress --exclude='.*' ${service} ${destination}"  
      rsync -av --progress --exclude='.*' ${service}" ${destination}"
    fi

    if [[ $? -eq 0 ]];
    then
      log "Success: rsync copied file."
      return 0
    else
      log -w "Error at rsync"
      return 1
    fi

}

deploy () {
    destination=${1}
    log "Deploy to ${destination}"
    service=$(cat Cargo.toml | grep name -m 1 | awk -F' = ' '{ print $2 }' | sed 's/.\(.*\)/\1/' | sed 's/\(.*\)./\1/')
    log "Serv ice in scope: ${service}"
    
    log "Git operations"
    if [[ ${dry_run} != "true" ]];
    then
        git add .
        git commit -m "Release"
        git push
    else 
        log " dry run: git add  . & git commit -m 'Release' & git push"
    fi 
    
    log "Cargo build --release"
    if [[ $dry_run != "true" ]];  
    then
       cargo build --release
    fi

    echo "Move build to server. Server password needed."
    cd ./target/release
    
    copy_file "./${service}" "${destination}"

    if [[ $? -eq 0 ]];
    then
        log "Increase version"
        log "cargo set-version --bump patch"
        if [[ $dry_run != "true" ]];  
        then
            cargo set-version --bump patch
        fi
    fi
    cd ${current_directory}
}

# ------------------ main execution -------------------------

echo "Use Option   -h for help. You will be asked for multiple times to enter password of server."

parameter_check "$@"
log "file_name: $file_name"
log "dry_run: $dry_run"
log "verbose: $verbose"
log "current_directory: $current_directory"
log "destination: $destination"

# script is part of the to-repository, therefor we need to get up a directory

if [[ -z ${destination} ]]; 
then 
  log -e "Destination location is not provided"
  usage
fi
  
deploy ${destination}

log "All done. "
exit 0;
