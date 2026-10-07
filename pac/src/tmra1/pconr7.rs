#[doc = "Register `PCONR7` reader"]
pub type R = crate::R<Pconr7Spec>;
#[doc = "Register `PCONR7` writer"]
pub type W = crate::W<Pconr7Spec>;
#[doc = "Field `STAC` reader - desc STAC"]
pub type StacR = crate::FieldReader;
#[doc = "Field `STAC` writer - desc STAC"]
pub type StacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STPC` reader - desc STPC"]
pub type StpcR = crate::FieldReader;
#[doc = "Field `STPC` writer - desc STPC"]
pub type StpcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPC` reader - desc CMPC"]
pub type CmpcR = crate::FieldReader;
#[doc = "Field `CMPC` writer - desc CMPC"]
pub type CmpcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERC` reader - desc PERC"]
pub type PercR = crate::FieldReader;
#[doc = "Field `PERC` writer - desc PERC"]
pub type PercW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORC` reader - desc FORC"]
pub type ForcR = crate::FieldReader;
#[doc = "Field `FORC` writer - desc FORC"]
pub type ForcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUTEN` reader - desc OUTEN"]
pub type OutenR = crate::BitReader;
#[doc = "Field `OUTEN` writer - desc OUTEN"]
pub type OutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc STAC"]
    #[inline(always)]
    pub fn stac(&self) -> StacR {
        StacR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc STPC"]
    #[inline(always)]
    pub fn stpc(&self) -> StpcR {
        StpcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc CMPC"]
    #[inline(always)]
    pub fn cmpc(&self) -> CmpcR {
        CmpcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc PERC"]
    #[inline(always)]
    pub fn perc(&self) -> PercR {
        PercR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc FORC"]
    #[inline(always)]
    pub fn forc(&self) -> ForcR {
        ForcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - desc OUTEN"]
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc STAC"]
    #[inline(always)]
    pub fn stac(&mut self) -> StacW<'_, Pconr7Spec> {
        StacW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc STPC"]
    #[inline(always)]
    pub fn stpc(&mut self) -> StpcW<'_, Pconr7Spec> {
        StpcW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc CMPC"]
    #[inline(always)]
    pub fn cmpc(&mut self) -> CmpcW<'_, Pconr7Spec> {
        CmpcW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc PERC"]
    #[inline(always)]
    pub fn perc(&mut self) -> PercW<'_, Pconr7Spec> {
        PercW::new(self, 6)
    }
    #[doc = "Bits 8:9 - desc FORC"]
    #[inline(always)]
    pub fn forc(&mut self) -> ForcW<'_, Pconr7Spec> {
        ForcW::new(self, 8)
    }
    #[doc = "Bit 12 - desc OUTEN"]
    #[inline(always)]
    pub fn outen(&mut self) -> OutenW<'_, Pconr7Spec> {
        OutenW::new(self, 12)
    }
}
#[doc = "desc PCONR7\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pconr7Spec;
impl crate::RegisterSpec for Pconr7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pconr7::R`](R) reader structure"]
impl crate::Readable for Pconr7Spec {}
#[doc = "`write(|w| ..)` method takes [`pconr7::W`](W) writer structure"]
impl crate::Writable for Pconr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCONR7 to value 0"]
impl crate::Resettable for Pconr7Spec {}
