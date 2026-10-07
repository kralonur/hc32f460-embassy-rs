#[doc = "Register `RCCR` reader"]
pub type R = crate::R<RccrSpec>;
#[doc = "Register `RCCR` writer"]
pub type W = crate::W<RccrSpec>;
#[doc = "Field `RDIVS` reader - desc RDIVS"]
pub type RdivsR = crate::FieldReader;
#[doc = "Field `RDIVS` writer - desc RDIVS"]
pub type RdivsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RCKS` reader - desc RCKS"]
pub type RcksR = crate::FieldReader;
#[doc = "Field `RCKS` writer - desc RCKS"]
pub type RcksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INEXS` reader - desc INEXS"]
pub type InexsR = crate::BitReader;
#[doc = "Field `INEXS` writer - desc INEXS"]
pub type InexsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNFS` reader - desc DNFS"]
pub type DnfsR = crate::FieldReader;
#[doc = "Field `DNFS` writer - desc DNFS"]
pub type DnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGES` reader - desc EDGES"]
pub type EdgesR = crate::FieldReader;
#[doc = "Field `EDGES` writer - desc EDGES"]
pub type EdgesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXREFE` reader - desc EXREFE"]
pub type ExrefeR = crate::BitReader;
#[doc = "Field `EXREFE` writer - desc EXREFE"]
pub type ExrefeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc RDIVS"]
    #[inline(always)]
    pub fn rdivs(&self) -> RdivsR {
        RdivsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:6 - desc RCKS"]
    #[inline(always)]
    pub fn rcks(&self) -> RcksR {
        RcksR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - desc INEXS"]
    #[inline(always)]
    pub fn inexs(&self) -> InexsR {
        InexsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc DNFS"]
    #[inline(always)]
    pub fn dnfs(&self) -> DnfsR {
        DnfsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc EDGES"]
    #[inline(always)]
    pub fn edges(&self) -> EdgesR {
        EdgesR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - desc EXREFE"]
    #[inline(always)]
    pub fn exrefe(&self) -> ExrefeR {
        ExrefeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc RDIVS"]
    #[inline(always)]
    pub fn rdivs(&mut self) -> RdivsW<'_, RccrSpec> {
        RdivsW::new(self, 0)
    }
    #[doc = "Bits 3:6 - desc RCKS"]
    #[inline(always)]
    pub fn rcks(&mut self) -> RcksW<'_, RccrSpec> {
        RcksW::new(self, 3)
    }
    #[doc = "Bit 7 - desc INEXS"]
    #[inline(always)]
    pub fn inexs(&mut self) -> InexsW<'_, RccrSpec> {
        InexsW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc DNFS"]
    #[inline(always)]
    pub fn dnfs(&mut self) -> DnfsW<'_, RccrSpec> {
        DnfsW::new(self, 8)
    }
    #[doc = "Bits 12:13 - desc EDGES"]
    #[inline(always)]
    pub fn edges(&mut self) -> EdgesW<'_, RccrSpec> {
        EdgesW::new(self, 12)
    }
    #[doc = "Bit 15 - desc EXREFE"]
    #[inline(always)]
    pub fn exrefe(&mut self) -> ExrefeW<'_, RccrSpec> {
        ExrefeW::new(self, 15)
    }
}
#[doc = "desc RCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`rccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccrSpec;
impl crate::RegisterSpec for RccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rccr::R`](R) reader structure"]
impl crate::Readable for RccrSpec {}
#[doc = "`write(|w| ..)` method takes [`rccr::W`](W) writer structure"]
impl crate::Writable for RccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCCR to value 0"]
impl crate::Resettable for RccrSpec {}
