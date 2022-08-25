# Random

# Users
echo "------------- USERS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 5; i++))
do
    # Generate a random string's character number
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 20)))
    # Generate a random string for wallet address

    $(near call dev-1661391008148-51376031507742 create_user "{\"name\": \"$RAN_STR1\", \"role\": \"Admin\", \"email\": \"$RAN_STR2@gmail.com\", \"blockchain_type\": \"Near\", \"wallet_address\": \"$RAN_STR3.testnet\"}" --deposit 0.5 --accountId duongnh.testnet > temp.txt)

    OUTPUT=$(grep 'Storage' ./temp.txt)
    SUBSTRING=$(echo $OUTPUT| cut -d':' -f 3)
    echo "create_user: $SUBSTRING yoctoNear" >> log-deposit.txt
done

echo "\n" >> log-deposit.txt

# Criterias
echo "------------- CRITERIAS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 5; i++))
do
    # Generate a random string's character number
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 10)))
    # Generate a random string for wallet address

    near call dev-1661391008148-51376031507742 create_criteria "{\"created_by\": 5, \"descriptions\": [\"$RAN_STR1\", \"$RAN_STR2\", \"$RAN_STR3\"]}" --deposit 0.5 --accountId duongnh.testnet > temp.txt

    OUTPUT=$(grep 'Storage' ./temp.txt)
    SUBSTRING=$(echo $OUTPUT| cut -d':' -f 3)
    echo "create_criteria: $SUBSTRING yoctoNear" >> log-deposit.txt
done

echo "\n" >> log-deposit.txt

# Options
echo "------------- ANSWER OPTIONS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 5; i++))
do
    # Generate a random string's character number
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR4=$(openssl rand -base64 $(($RANDOM % 10)))
    # Generate a random string for wallet address

    near call dev-1661391008148-51376031507742 create_poll_option "{\"created_by\": 5, \"title\": \"$RAN_STR1\", \"description\": \"$RAN_STR2\", \"options\": [\"$RAN_STR3\", \"$RAN_STR4\"]}" --deposit 0.5 --accountId duongnh.testnet > temp.txt

    OUTPUT=$(grep 'Storage' ./temp.txt)
    SUBSTRING=$(echo $OUTPUT| cut -d':' -f 3)
    echo "create_poll_option: $SUBSTRING yoctoNear" >> log-deposit.txt
done

echo "\n" >> log-deposit.txt

# Polls
echo "------------- POLLS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 5; i++))
do
    # Generate a random string's character number
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 10)))
    # Generate a random string for wallet address

    near call dev-1661391008148-51376031507742 create_poll "{\"criteria_option_id_array\": [{\"criteria_id\": 5, \"poll_option_id\": 5}, {\"criteria_id\": 2, \"poll_option_id\": 2}], \"created_by\": 5, \"img_url\": \"$RAN_STR1\", \"title\": \"$RAN_STR2\", \"description\": \"$RAN_STR3\", \"start_at\": 0, \"end_at\": 0}" --deposit 0.5 --accountId duongnh.testnet > temp.txt

    SUBSTRING=$(grep 'Storage' ./temp.txt)
    echo "create_poll: $SUBSTRING yoctoNear" >> log-deposit.txt
done

# Remove temporary file
rm temp.txt