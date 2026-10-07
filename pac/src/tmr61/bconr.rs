#[doc = "Register `BCONR` reader"]
pub type R = crate::R<BconrSpec>;
#[doc = "Register `BCONR` writer"]
pub type W = crate::W<BconrSpec>;
#[doc = "Field `BENA` reader - desc BENA"]
pub type BenaR = crate::BitReader;
#[doc = "Field `BENA` writer - desc BENA"]
pub type BenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSEA` reader - desc BSEA"]
pub type BseaR = crate::BitReader;
#[doc = "Field `BSEA` writer - desc BSEA"]
pub type BseaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BENB` reader - desc BENB"]
pub type BenbR = crate::BitReader;
#[doc = "Field `BENB` writer - desc BENB"]
pub type BenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSEB` reader - desc BSEB"]
pub type BsebR = crate::BitReader;
#[doc = "Field `BSEB` writer - desc BSEB"]
pub type BsebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BENP` reader - desc BENP"]
pub type BenpR = crate::BitReader;
#[doc = "Field `BENP` writer - desc BENP"]
pub type BenpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSEP` reader - desc BSEP"]
pub type BsepR = crate::BitReader;
#[doc = "Field `BSEP` writer - desc BSEP"]
pub type BsepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BENSPA` reader - desc BENSPA"]
pub type BenspaR = crate::BitReader;
#[doc = "Field `BENSPA` writer - desc BENSPA"]
pub type BenspaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSESPA` reader - desc BSESPA"]
pub type BsespaR = crate::BitReader;
#[doc = "Field `BSESPA` writer - desc BSESPA"]
pub type BsespaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTRUSPA` reader - desc BTRUSPA"]
pub type BtruspaR = crate::BitReader;
#[doc = "Field `BTRUSPA` writer - desc BTRUSPA"]
pub type BtruspaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTRDSPA` reader - desc BTRDSPA"]
pub type BtrdspaR = crate::BitReader;
#[doc = "Field `BTRDSPA` writer - desc BTRDSPA"]
pub type BtrdspaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BENSPB` reader - desc BENSPB"]
pub type BenspbR = crate::BitReader;
#[doc = "Field `BENSPB` writer - desc BENSPB"]
pub type BenspbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSESPB` reader - desc BSESPB"]
pub type BsespbR = crate::BitReader;
#[doc = "Field `BSESPB` writer - desc BSESPB"]
pub type BsespbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTRUSPB` reader - desc BTRUSPB"]
pub type BtruspbR = crate::BitReader;
#[doc = "Field `BTRUSPB` writer - desc BTRUSPB"]
pub type BtruspbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTRDSPB` reader - desc BTRDSPB"]
pub type BtrdspbR = crate::BitReader;
#[doc = "Field `BTRDSPB` writer - desc BTRDSPB"]
pub type BtrdspbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BENA"]
    #[inline(always)]
    pub fn bena(&self) -> BenaR {
        BenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BSEA"]
    #[inline(always)]
    pub fn bsea(&self) -> BseaR {
        BseaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BENB"]
    #[inline(always)]
    pub fn benb(&self) -> BenbR {
        BenbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BSEB"]
    #[inline(always)]
    pub fn bseb(&self) -> BsebR {
        BsebR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BENP"]
    #[inline(always)]
    pub fn benp(&self) -> BenpR {
        BenpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc BSEP"]
    #[inline(always)]
    pub fn bsep(&self) -> BsepR {
        BsepR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc BENSPA"]
    #[inline(always)]
    pub fn benspa(&self) -> BenspaR {
        BenspaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc BSESPA"]
    #[inline(always)]
    pub fn bsespa(&self) -> BsespaR {
        BsespaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - desc BTRUSPA"]
    #[inline(always)]
    pub fn btruspa(&self) -> BtruspaR {
        BtruspaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc BTRDSPA"]
    #[inline(always)]
    pub fn btrdspa(&self) -> BtrdspaR {
        BtrdspaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - desc BENSPB"]
    #[inline(always)]
    pub fn benspb(&self) -> BenspbR {
        BenspbR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc BSESPB"]
    #[inline(always)]
    pub fn bsespb(&self) -> BsespbR {
        BsespbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - desc BTRUSPB"]
    #[inline(always)]
    pub fn btruspb(&self) -> BtruspbR {
        BtruspbR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc BTRDSPB"]
    #[inline(always)]
    pub fn btrdspb(&self) -> BtrdspbR {
        BtrdspbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BENA"]
    #[inline(always)]
    pub fn bena(&mut self) -> BenaW<'_, BconrSpec> {
        BenaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc BSEA"]
    #[inline(always)]
    pub fn bsea(&mut self) -> BseaW<'_, BconrSpec> {
        BseaW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BENB"]
    #[inline(always)]
    pub fn benb(&mut self) -> BenbW<'_, BconrSpec> {
        BenbW::new(self, 2)
    }
    #[doc = "Bit 3 - desc BSEB"]
    #[inline(always)]
    pub fn bseb(&mut self) -> BsebW<'_, BconrSpec> {
        BsebW::new(self, 3)
    }
    #[doc = "Bit 8 - desc BENP"]
    #[inline(always)]
    pub fn benp(&mut self) -> BenpW<'_, BconrSpec> {
        BenpW::new(self, 8)
    }
    #[doc = "Bit 9 - desc BSEP"]
    #[inline(always)]
    pub fn bsep(&mut self) -> BsepW<'_, BconrSpec> {
        BsepW::new(self, 9)
    }
    #[doc = "Bit 16 - desc BENSPA"]
    #[inline(always)]
    pub fn benspa(&mut self) -> BenspaW<'_, BconrSpec> {
        BenspaW::new(self, 16)
    }
    #[doc = "Bit 17 - desc BSESPA"]
    #[inline(always)]
    pub fn bsespa(&mut self) -> BsespaW<'_, BconrSpec> {
        BsespaW::new(self, 17)
    }
    #[doc = "Bit 20 - desc BTRUSPA"]
    #[inline(always)]
    pub fn btruspa(&mut self) -> BtruspaW<'_, BconrSpec> {
        BtruspaW::new(self, 20)
    }
    #[doc = "Bit 21 - desc BTRDSPA"]
    #[inline(always)]
    pub fn btrdspa(&mut self) -> BtrdspaW<'_, BconrSpec> {
        BtrdspaW::new(self, 21)
    }
    #[doc = "Bit 24 - desc BENSPB"]
    #[inline(always)]
    pub fn benspb(&mut self) -> BenspbW<'_, BconrSpec> {
        BenspbW::new(self, 24)
    }
    #[doc = "Bit 25 - desc BSESPB"]
    #[inline(always)]
    pub fn bsespb(&mut self) -> BsespbW<'_, BconrSpec> {
        BsespbW::new(self, 25)
    }
    #[doc = "Bit 28 - desc BTRUSPB"]
    #[inline(always)]
    pub fn btruspb(&mut self) -> BtruspbW<'_, BconrSpec> {
        BtruspbW::new(self, 28)
    }
    #[doc = "Bit 29 - desc BTRDSPB"]
    #[inline(always)]
    pub fn btrdspb(&mut self) -> BtrdspbW<'_, BconrSpec> {
        BtrdspbW::new(self, 29)
    }
}
#[doc = "desc BCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BconrSpec;
impl crate::RegisterSpec for BconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bconr::R`](R) reader structure"]
impl crate::Readable for BconrSpec {}
#[doc = "`write(|w| ..)` method takes [`bconr::W`](W) writer structure"]
impl crate::Writable for BconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCONR to value 0"]
impl crate::Resettable for BconrSpec {}
