import { Cw721NftInfoResponse } from '../../bindings/models';

export function getPortraitAirdropMetadata(
  tokenId: string
): Cw721NftInfoResponse {
  return {
    token_uri:
      'ipfs://bafybeibxqt3hfce4phwzoixkj4kyxxit2wkznsylcim6pwlntq4frd3rri',
    extension: {
      image:
        'ipfs://bafybeibxqt3hfce4phwzoixkj4kyxxit2wkznsylcim6pwlntq4frd3rri',
      image_data: null,
      external_url: null,
      description:
        'The Meeting of the Galactics was designed by Luna Millionaire Portrait and captures the meeting of First Galactic Settlers and Explorers as they discuss the formation of GalacticDAO. The portrait was airdropped on Oct 16th, the same weekend the first version of the DAO was announced. This was given to xxx.',
      name: `Meeting of the Galactics #${tokenId}`,
      attributes: null,
      background_color: null,
      animation_url: null,
      youtube_url: null,
    },
  };
}

export function getComicAirdropMetadata(tokenId: string): Cw721NftInfoResponse {
  return {
    token_uri:
      'ipfs://bafybeiaiqqzbe2fvv5n54fvaux2uox3t34ekedurbqaulelv3ewvhagnui/gp_comic_vol1.pdf',
    extension: {
      image:
        'ipfs://bafybeid6tkzvkuyyitderfsfh7nuxtu3zu643z5lubcfojeh2bftd7a3z4/gp_comic_vol1_cover.jpg',
      image_data: null,
      external_url: null,
      description:
        'A GalacticDAO community comic by @touch_the_sky1, @ilcampedelli & @nft_luna (their Twitter handles, make sure to drop them a follow) - A new threat to the Terraverse. An unlikely hero. Will she rise in glory to save the day? Or will she crumble under the pressure of the impending peril?',
      name: 'The Galactic Punks Comic: Volume 1',
      attributes: null,
      background_color: null,
      animation_url: null,
      youtube_url: null,
    },
  };
}

export function getGlitchAirdropMetadata(
  tokenId: string
): Cw721NftInfoResponse {
  return {
    token_uri:
      'ipfs://bafybeiawqw4rr2fg7oavwjiirpzxaybcaw4ats3uvcfswl2c3n2hrqfy54',
    extension: {
      image:
        'ipfs://bafybeiawqw4rr2fg7oavwjiirpzxaybcaw4ats3uvcfswl2c3n2hrqfy54',
      image_data: null,
      external_url: null,
      description:
        'The Genesis Glitch is a unique celebration of the Galactic Punks mint, showcasing every punk made. A limited supply was airdropped to every Galactic Glitch holder on Oct 16th. This was given to xxx.',
      name: `Genesis Glitch #${tokenId}`,
      attributes: null,
      background_color: null,
      animation_url: null,
      youtube_url: null,
    },
  };
}
