#[doc = "Register `BLKGPCON` reader"]
pub type R = crate::R<BlkgpconSpec>;
#[doc = "Register `BLKGPCON` writer"]
pub type W = crate::W<BlkgpconSpec>;
#[doc = "Field `SABGR` reader - desc SABGR"]
pub type SabgrR = crate::BitReader;
#[doc = "Field `SABGR` writer - desc SABGR"]
pub type SabgrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - desc CR"]
pub type CrR = crate::BitReader;
#[doc = "Field `CR` writer - desc CR"]
pub type CrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWC` reader - desc RWC"]
pub type RwcR = crate::BitReader;
#[doc = "Field `RWC` writer - desc RWC"]
pub type RwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IABG` reader - desc IABG"]
pub type IabgR = crate::BitReader;
#[doc = "Field `IABG` writer - desc IABG"]
pub type IabgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SABGR"]
    #[inline(always)]
    pub fn sabgr(&self) -> SabgrR {
        SabgrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CR"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RWC"]
    #[inline(always)]
    pub fn rwc(&self) -> RwcR {
        RwcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc IABG"]
    #[inline(always)]
    pub fn iabg(&self) -> IabgR {
        IabgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SABGR"]
    #[inline(always)]
    pub fn sabgr(&mut self) -> SabgrW<'_, BlkgpconSpec> {
        SabgrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, BlkgpconSpec> {
        CrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RWC"]
    #[inline(always)]
    pub fn rwc(&mut self) -> RwcW<'_, BlkgpconSpec> {
        RwcW::new(self, 2)
    }
    #[doc = "Bit 3 - desc IABG"]
    #[inline(always)]
    pub fn iabg(&mut self) -> IabgW<'_, BlkgpconSpec> {
        IabgW::new(self, 3)
    }
}
#[doc = "desc BLKGPCON\n\nYou can [`read`](crate::Reg::read) this register and get [`blkgpcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blkgpcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkgpconSpec;
impl crate::RegisterSpec for BlkgpconSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`blkgpcon::R`](R) reader structure"]
impl crate::Readable for BlkgpconSpec {}
#[doc = "`write(|w| ..)` method takes [`blkgpcon::W`](W) writer structure"]
impl crate::Writable for BlkgpconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLKGPCON to value 0"]
impl crate::Resettable for BlkgpconSpec {}
