# https://docs.aws.amazon.com/lambda/latest/dg/python-handler.html
import os
import json
import boto3
from boto3.dynamodb.conditions import Key

def lambda_handler(event, context):
  
    print(event['queryStringParameters'])
    json_region = os.environ['AWS_REGION']
    STATION_ID_VALUE = event['queryStringParameters']['station_id']
    
    TABLE_NAME = "weatherstation_data"
    
    # Creating the DynamoDB Client
    dynamodb_client = boto3.client('dynamodb', region_name="us-west-2")
    
    # Creating the DynamoDB Table Resource
    dynamodb = boto3.resource('dynamodb', region_name="us-west-2")
    table = dynamodb.Table(TABLE_NAME)
    
    response = dynamodb_client.query(
        TableName=TABLE_NAME,
        KeyConditionExpression='station_id = :artist',
        ExpressionAttributeValues={
            ':artist': {'S': STATION_ID_VALUE}
        }
    );
    
    return {
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": json.dumps({
            "dataset": response
        })
    }