import ethers from 'ethers';
import fs from 'fs';
import dotenv from 'dotenv';
dotenv.config();

const DAI_CONTRACT_ADDRESS = "0x92d13090891f5bf92fa7f275a9a69df5e566fb84";
const INFURA = new ethers.providers.InfuraProvider('kovan');
const DAI_ABI = JSON.parse(fs.readFileSync("mintableTokenABI.json", "utf8"))
const WALLET = new ethers.Wallet(process.env.ETHEREUM_PRIVATE_KEY, INFURA);
let DAI = new ethers.Contract(DAI_CONTRACT_ADDRESS, DAI_ABI, WALLET);
async function sendDai(to, amount){
  return await DAI.transfer(to, ethers.utils.parseEther(amount.toString(), "eth"))
};
(async () => {
  let tx = await sendDai("0xF238247192c411d7f0d31049B85036E6DC81B368", 1);
  console.log(`https://kovan.etherscan.io/tx/${tx.hash}`)
})();
