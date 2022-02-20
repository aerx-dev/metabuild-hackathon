How to stop the local near container:

          1. Run 'kurtosis enclave ls'
          2. Copy the enclave ID that your NEAR cluster is running inside
          3. Run 'kurtosis enclave stop ENCLAVE_ID_YOU_COPIED'

To remove stopped clusters, run 'kurtosis clean'. You can also run 'kurtosis clean -a' to stop & remove _all_ clusters, including running ones.
