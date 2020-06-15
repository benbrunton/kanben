const AWS = require('aws-sdk');

module.exports.hello = async event => {
  return {
    statusCode: 200,
    body: JSON.stringify(
      {
        message: 'Go Serverless v1.0! Your function executed successfully!',
        input: event,
      },
      null,
      2
    ),
  };
};

module.exports.getPutUrl = async event => {
  let eventBody = null;
  try {
    eventBody = JSON.parse(event.body);
    if(eventBody === null) { throw new Error("null body"); }
  } catch (err) {
    return { statusCode: 400 };
  }
  const { filename } = eventBody;
  const s3 = new AWS.S3();
  const signedUrl = s3.getSignedUrl('putObject', {
    Bucket: process.env["BUCKET_NAME"],
    Key: `uploads/${filename}`,
  });
  return {
    statusCode: 200,
    body: JSON.stringify(
      {
        message: 'signed put url',
        url: signedUrl
      }
    )
  };
};
