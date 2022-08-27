# Random

# Users
echo "------------- USERS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 10; i++))
do
    # Generate a random string's character number
    # -> Generate a random string for params
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 20)))

    $(near call dev-1661499707374-98283221529225 create_user "{\"name\": \"$RAN_STR1\", \"role\": \"Admin\", \"email\": \"$RAN_STR2@gmail.com\", \"blockchain_type\": \"Near\", \"wallet_address\": \"$RAN_STR3.testnet\"}" --deposit 0.5 --accountId duongnh.testnet > temp.txt)

    # Extract storage data + fee from Contract Call Output
    OUTPUT_FEE=$(grep 'Storage fee' ./temp.txt)
    OUTPUT_DATA=$(grep 'Storage data' ./temp.txt)
    SUBSTRING_FEE=$(echo $OUTPUT_FEE| cut -d':' -f 3)
    SUBSTRING_DATA=$(echo $OUTPUT_DATA| cut -d':' -f 3)

    echo "create_user: $SUBSTRING_DATA bytes - $SUBSTRING_FEE yoctoNear" >> log-deposit.txt
done

echo "\n" >> log-deposit.txt

# Criterias
echo "------------- CRITERIAS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 10; i++))
do
    # Generate a random string's character number
    # -> Generate a random string for params
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 10)))

    near call dev-1661499707374-98283221529225 create_criteria "{\"created_by\": 5, \"descriptions\": [\"$RAN_STR1\", \"$RAN_STR2\", \"$RAN_STR3\"]}" --deposit 0.5 --accountId duongnh.testnet > temp.txt

    # Extract storage data + fee from Contract Call Output
    OUTPUT_FEE=$(grep 'Storage fee' ./temp.txt)
    OUTPUT_DATA=$(grep 'Storage data' ./temp.txt)
    SUBSTRING_FEE=$(echo $OUTPUT_FEE| cut -d':' -f 3)
    SUBSTRING_DATA=$(echo $OUTPUT_DATA| cut -d':' -f 3)

    echo "create_criteria: $SUBSTRING_DATA bytes - $SUBSTRING_FEE yoctoNear" >> log-deposit.txt
done

echo "\n" >> log-deposit.txt

# Options
echo "------------- ANSWER OPTIONS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 10; i++))
do
    # Generate a random string's character number
    # -> Generate a random string for params
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR4=$(openssl rand -base64 $(($RANDOM % 10)))

    near call dev-1661499707374-98283221529225 create_poll_option "{\"created_by\": 5, \"title\": \"$RAN_STR1\", \"description\": \"$RAN_STR2\", \"options\": [\"$RAN_STR3\", \"$RAN_STR4\"]}" --deposit 0.5 --accountId duongnh.testnet > temp.txt

    # Extract storage data + fee from Contract Call Output
    OUTPUT_FEE=$(grep 'Storage fee' ./temp.txt)
    OUTPUT_DATA=$(grep 'Storage data' ./temp.txt)
    SUBSTRING_FEE=$(echo $OUTPUT_FEE| cut -d':' -f 3)
    SUBSTRING_DATA=$(echo $OUTPUT_DATA| cut -d':' -f 3)

    echo "create_poll_option: $SUBSTRING_DATA bytes - $SUBSTRING_FEE yoctoNear" >> log-deposit.txt
done

echo "\n" >> log-deposit.txt

# Polls
echo "------------- POLLS STORAGE FEE -------------" >> log-deposit.txt
for((i = 0; i < 10; i++))
do
    # Generate a random string's character number
    # -> Generate a random string for params
    RAN_STR1=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR2=$(openssl rand -base64 $(($RANDOM % 10)))
    RAN_STR3=$(openssl rand -base64 $(($RANDOM % 10)))

    near call dev-1661499707374-98283221529225 create_poll "{\"criteria_option_id_array\": [{\"criteria_id\": 5, \"poll_option_id\": 5}, {\"criteria_id\": 2, \"poll_option_id\": 2}], \"created_by\": 5, \"img_url\": \"$RAN_STR1\", \"title\": \"$RAN_STR2\", \"description\": \"$RAN_STR3\", \"start_at\": 0, \"end_at\": 0}" --deposit 0.5 --accountId duongnh.testnet > temp.txt

    # Extract storage data + fee from Contract Call Output
    OUTPUT_FEE=$(grep 'Storage fee' ./temp.txt)
    echo "create_poll: $OUTPUT_FEE yoctoNear" >> log-deposit.txt
done

# Remove temporary file
rm temp.txt