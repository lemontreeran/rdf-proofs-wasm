import {
    ellipticElGamalKeyGen,
    initializeWasm,
    getEncryptedUid,
} from '../../lib';

describe('ElGamal', () => {
    test('ellipticElGamalKeyGen', async () => {
        await initializeWasm();
      
        const keyPair = ellipticElGamalKeyGen();
        console.log(keyPair);
  
        expect(keyPair).toBeTruthy();
    });

    test('encrypt Uid', async() => {
        await initializeWasm();

        const encryptedUid = getEncryptedUid(new Uint8Array(Buffer.from('USER-ID')));
        console.log(encryptedUid)
        
        expect(encryptedUid).toBeTruthy();
    });
});