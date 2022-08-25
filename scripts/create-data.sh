# Create Users
near call dev-1661391008148-51376031507742 create_user '{"name": "Zuong", "role": "Admin", "email": "muoi07052001@gmail.com", "blockchain_type": "Near", "wallet_address": "duongnh.testnet"}' --deposit 0.1 --accountId duongnh.testnet
near call dev-1661391008148-51376031507742 create_user '{"name": "Nguyen Xuan Hai", "role": "Admin", "email": "hainx@gmail.com", "blockchain_type": "Near", "wallet_address": "hainx.testnet"}' --deposit 0.1 --accountId duongnh.testnet
near call dev-1661391008148-51376031507742 create_user '{"name": "Nguyen Duc Toan", "role": "Employee", "email": "toannd@gmail.com", "blockchain_type": "Near", "wallet_address": "toannd.testnet"}' --deposit 0.1 --accountId duongnh.testnet
near call dev-1661391008148-51376031507742 create_user '{"name": "Phi Duc Binh", "role": "Employee", "email": "binhpd@gmail.com", "blockchain_type": "Near", "wallet_address": "binhpd.testnet"}' --deposit 0.1 --accountId duongnh.testnet

# Create Criterias
near call dev-1661391008148-51376031507742 create_criteria '{"created_by": 1, "descriptions": ["The best food", "The best drink"]}' --deposit 0.1 --accountId duongnh.testnet
near call dev-1661391008148-51376031507742 create_criteria '{"created_by": 2, "descriptions": ["The most handsome employee", "The most creative employee"]}' --deposit 0.1 --accountId duongnh.testnet

# Create Poll Options
near call dev-1661391008148-51376031507742 create_poll_option '{"created_by": 1, "title": "The best food", "description": "The best food for developers", "options": ["Hamburger", "Chicken"]}' --deposit 0.1 --accountId duongnh.testnet
near call dev-1661391008148-51376031507742 create_poll_option '{"created_by": 1, "title": "The best drink", "description": "The best drink for developers", "options": ["Coca", "Pepsi", "Fanta"]}' --deposit 0.1 --accountId duongnh.testnet

# Create Polls
near call dev-1661391008148-51376031507742 create_poll '{"criteria_option_id_array": [{"criteria_id": 1, "poll_option_id": 1}, {"criteria_id": 2, "poll_option_id": 2}], "created_by": 1, "img_url": "", "title": "How to keep energy", "description": "Things that developers love", "start_at": 0, "end_at": 0}' --deposit 0.1 --accountId duongnh.testnet

# Get all results
near view dev-1661391008148-51376031507742 get_all_results '{"from_index": 0, "limit": 100}'
near view dev-1661391008148-51376031507742 get_all_results_by_poll_criteria_id '{"poll_id": 1, "criteria_id": 1}'